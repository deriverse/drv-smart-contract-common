use std::{collections::HashMap, path::Path};

// ── Constant + enum resolution from types.rs ─────────────────────────────────

/// `account_type::X = N` constants → `"X" -> N`
fn parse_account_type_constants(types_rs: &syn::File) -> HashMap<String, u32> {
    for item in &types_rs.items {
        if let syn::Item::Mod(m) = item {
            if m.ident == "account_type" {
                if let Some((_, items)) = &m.content {
                    return items
                        .iter()
                        .filter_map(|item| {
                            let c = match item {
                                syn::Item::Const(c) => c,
                                _ => return None,
                            };
                            let name = c.ident.to_string();
                            let value = lit_u32(c.expr.as_ref())?;
                            Some((name, value))
                        })
                        .collect();
                }
            }
        }
    }
    HashMap::new()
}

/// `AccountType` enum discriminants → `N -> "VariantName"` (e.g. 14 -> "SpotBidsTree")
fn parse_account_type_enum(types_rs: &syn::File) -> HashMap<u32, String> {
    for item in &types_rs.items {
        if let syn::Item::Mod(m) = item {
            if m.ident == "account_type" {
                if let Some((_, items)) = &m.content {
                    for item in items {
                        if let syn::Item::Enum(e) = item {
                            if e.ident == "AccountType" {
                                return e
                                    .variants
                                    .iter()
                                    .filter_map(|v| {
                                        let (_, expr) = v.discriminant.as_ref()?;
                                        let value = lit_u32(expr)?;
                                        Some((value, v.ident.to_string()))
                                    })
                                    .collect();
                            }
                        }
                    }
                }
            }
        }
    }
    HashMap::new()
}

fn lit_u32(expr: &syn::Expr) -> Option<u32> {
    match expr {
        syn::Expr::Lit(el) => match &el.lit {
            syn::Lit::Int(n) => n.base10_parse().ok(),
            _ => None,
        },
        _ => None,
    }
}

// ── Resolve constant expressions (literals or `account_type::X` paths) ───────

fn eval_u32_expr(expr: &syn::Expr, consts: &HashMap<String, u32>) -> Option<u32> {
    match expr {
        syn::Expr::Lit(_) => lit_u32(expr),
        syn::Expr::Path(ep) => {
            let last = ep.path.segments.last()?.ident.to_string();
            consts.get(&last).copied()
        }
        syn::Expr::Block(eb) => {
            if eb.block.stmts.len() == 1 {
                if let syn::Stmt::Expr(inner, _) = &eb.block.stmts[0] {
                    return eval_u32_expr(inner, consts);
                }
            }
            None
        }
        _ => None,
    }
}

// ── PDA entry ────────────────────────────────────────────────────────────────

struct PdaEntry {
    name: String,
    seeds: Vec<serde_json::Value>,
}

// ── Parse impl DrvAccount for X ──────────────────────────────────────────────

fn parse_drv_account_impls(
    pda_rs: &syn::File,
    consts: &HashMap<String, u32>,
    enum_names: &HashMap<u32, String>,
) -> Vec<PdaEntry> {
    pda_rs
        .items
        .iter()
        .filter_map(|item| {
            let imp = match item {
                syn::Item::Impl(i) => i,
                _ => return None,
            };
            // Must be `impl DrvAccount for X`
            let (_, trait_path, _) = imp.trait_.as_ref()?;
            if trait_path.segments.last()?.ident != "DrvAccount" {
                return None;
            }

            let name = pda_name(imp.self_ty.as_ref(), consts, enum_names)?;
            let seeds = impl_seeds(imp, consts)?;
            Some(PdaEntry { name, seeds })
        })
        .collect()
}

/// Derive a human-readable IDL name from the self type.
/// - `HolderAccountHeader`                              → "HolderAccountHeader"
/// - `SpotTradeAccountHeader<{ account_type::SPOT_BIDS_TREE }>` → "SpotBidsTree"
fn pda_name(
    ty: &syn::Type,
    consts: &HashMap<String, u32>,
    enum_names: &HashMap<u32, String>,
) -> Option<String> {
    let tp = match ty {
        syn::Type::Path(tp) => tp,
        _ => return None,
    };
    let seg = tp.path.segments.last()?;
    let base = seg.ident.to_string();

    match &seg.arguments {
        syn::PathArguments::None => Some(base),
        syn::PathArguments::AngleBracketed(args) => {
            // Generic: resolve the const param to a u32 then look up enum name.
            let arg = args.args.first()?;
            let tag = match arg {
                syn::GenericArgument::Const(expr) => eval_u32_expr(expr, consts),
                _ => None,
            }?;
            enum_names.get(&tag).cloned().or_else(|| Some(format!("{base}_{tag}")))
        }
        _ => None,
    }
}

/// Extract IDL seeds from `const PDA: PdaRef = PdaRef::new(&[...]);`.
fn impl_seeds(
    imp: &syn::ItemImpl,
    consts: &HashMap<String, u32>,
) -> Option<Vec<serde_json::Value>> {
    for item in &imp.items {
        if let syn::ImplItem::Const(c) = item {
            if c.ident == "PDA" {
                return seeds_from_expr(&c.expr, consts);
            }
        }
    }
    None
}

fn seeds_from_expr(
    expr: &syn::Expr,
    consts: &HashMap<String, u32>,
) -> Option<Vec<serde_json::Value>> {
    // PdaRef::new(&[seed1, seed2, ...])
    let call = match expr {
        syn::Expr::Call(c) => c,
        _ => return None,
    };
    let arg = call.args.first()?;
    let array = match arg {
        syn::Expr::Reference(r) => match r.expr.as_ref() {
            syn::Expr::Array(a) => a,
            _ => return None,
        },
        _ => return None,
    };
    array.elems.iter().map(|e| seed_to_json(e, consts)).collect()
}

fn seed_to_json(expr: &syn::Expr, consts: &HashMap<String, u32>) -> Option<serde_json::Value> {
    let call = match expr {
        syn::Expr::Call(c) => c,
        _ => return None,
    };
    let variant = match call.func.as_ref() {
        syn::Expr::Path(ep) => ep.path.segments.last()?.ident.to_string(),
        _ => return None,
    };
    let arg = call.args.first()?;

    match variant.as_str() {
        "ConstAscii" => {
            let s = lit_str(arg)?;
            let bytes: Vec<u8> = s.into_bytes();
            Some(serde_json::json!({ "kind": "const", "value": bytes }))
        }
        "ConstU32" => {
            let v = eval_u32_expr(arg, consts)?;
            let bytes: Vec<u8> = v.to_le_bytes().to_vec();
            Some(serde_json::json!({ "kind": "const", "value": bytes }))
        }
        "Param" => {
            let name = lit_str(arg)?;
            Some(serde_json::json!({ "kind": "param", "name": name, "type": "pubkey" }))
        }
        "ParamU32" => {
            let name = lit_str(arg)?;
            Some(serde_json::json!({ "kind": "param", "name": name, "type": "u32" }))
        }
        _ => None,
    }
}

fn lit_str(expr: &syn::Expr) -> Option<String> {
    match expr {
        syn::Expr::Lit(el) => match &el.lit {
            syn::Lit::Str(s) => Some(s.value()),
            _ => None,
        },
        _ => None,
    }
}

// ── Seed helpers (mirror the Python script's seed_* functions) ───────────────

fn seed_const_ascii(text: &str) -> serde_json::Value {
    let bytes: Vec<u8> = text.bytes().collect();
    serde_json::json!({ "kind": "const", "value": bytes })
}

fn seed_const_u32(value: u32) -> serde_json::Value {
    let bytes: Vec<u8> = value.to_le_bytes().to_vec();
    serde_json::json!({ "kind": "const", "value": bytes })
}

fn seed_account(path: &str) -> serde_json::Value {
    serde_json::json!({ "kind": "account", "path": path })
}

fn seed_account_typed(path: &str, account: &str) -> serde_json::Value {
    serde_json::json!({ "kind": "account", "path": path, "account": account })
}

fn seed_arg(path: &str) -> serde_json::Value {
    serde_json::json!({ "kind": "arg", "path": path })
}

/// Associated Token Program ID as const seed bytes.
fn seed_associated_token_program() -> serde_json::Value {
    let bytes: Vec<u8> = vec![
        140, 151, 37, 143, 78, 36, 137, 241, 187, 61, 16, 41, 20, 142, 13, 131,
        11, 90, 19, 153, 218, 255, 16, 132, 4, 142, 123, 216, 219, 233, 248, 89,
    ];
    serde_json::json!({ "kind": "const", "value": bytes })
}

// ── Account tag constants (mirrors Python script) ────────────────────────────

const ROOT_ACCOUNT_TAG:                   u32 = 2;
const INSTR_ACCOUNT_TAG:                  u32 = 7;
const SPOT_MAPS_ACCOUNT_TAG:              u32 = 10;
const SPOT_CLIENT_INFOS_ACCOUNT_TAG:      u32 = 12;
const SPOT_BIDS_TREE_ACCOUNT_TAG:         u32 = 14;
const SPOT_ASKS_TREE_ACCOUNT_TAG:         u32 = 15;
const SPOT_BID_ORDERS_ACCOUNT_TAG:        u32 = 16;
const SPOT_ASK_ORDERS_ACCOUNT_TAG:        u32 = 17;
const SPOT_LINES_ACCOUNT_TAG:             u32 = 18;
const COMMUNITY_ACCOUNT_TAG:              u32 = 34;
const CLIENT_COMMUNITY_ACCOUNT_TAG:       u32 = 35;
const PERP_ASK_ORDERS_ACCOUNT_TAG:        u32 = 36;
const PERP_ASKS_TREE_ACCOUNT_TAG:         u32 = 37;
const PERP_BID_ORDERS_ACCOUNT_TAG:        u32 = 38;
const PERP_BIDS_TREE_ACCOUNT_TAG:         u32 = 39;
const PERP_CLIENT_INFOS_ACCOUNT_TAG:      u32 = 41;
const PERP_CLIENT_INFOS2_ACCOUNT_TAG:     u32 = 42;
const PERP_CLIENT_INFOS3_ACCOUNT_TAG:     u32 = 43;
const PERP_CLIENT_INFOS4_ACCOUNT_TAG:     u32 = 44;
const PERP_CLIENT_INFOS5_ACCOUNT_TAG:     u32 = 45;
const PERP_LINES_ACCOUNT_TAG:             u32 = 46;
const PERP_MAPS_ACCOUNT_TAG:             u32 = 47;
const PERP_LONG_PX_TREE_ACCOUNT_TAG:      u32 = 48;
const PERP_SHORT_PX_TREE_ACCOUNT_TAG:     u32 = 49;
const PERP_REBALANCE_TIME_TREE_ACCOUNT_TAG: u32 = 50;
const VM_CLIENT_ACCOUNT_TAG:              u32 = 52;
const CLIENT_PRIMARY_ACCOUNT_TAG:         u32 = 31;

// ── Inline PDA mappings (mirrors Python inline_pda_mappings + apply_inline_pdas) ──

struct InlinePda {
    instruction: &'static str,
    account: &'static str,
    pda: serde_json::Value,
}

fn inline_pda_mappings() -> Vec<InlinePda> {
    let mut m = Vec::new();

    // holderAccount
    m.push(InlinePda {
        instruction: "NewHolder",
        account: "holderAccount",
        pda: serde_json::json!({ "seeds": [seed_const_ascii("drvs001"), seed_account("holderAdmin")] }),
    });
    m.push(InlinePda {
        instruction: "NewOperator",
        account: "holderAccount",
        pda: serde_json::json!({ "seeds": [seed_const_ascii("drvs001"), seed_account("holderAdmin")] }),
    });
    m.push(InlinePda {
        instruction: "NewRootAccount",
        account: "holderAccount",
        pda: serde_json::json!({ "seeds": [seed_const_ascii("drvs001"), seed_account("admin")] }),
    });

    // deriverseAuthority
    for ix in ["NewRootAccount", "NewInstrument"] {
        m.push(InlinePda {
            instruction: ix,
            account: "deriverseAuthority",
            pda: serde_json::json!({ "seeds": [seed_const_ascii("ndxnt")] }),
        });
    }

    // root + community for NewRootAccount
    m.push(InlinePda {
        instruction: "NewRootAccount",
        account: "root",
        pda: serde_json::json!({
            "seeds": [
                seed_arg("newRootAccountIdlArgs.version"),
                seed_const_u32(ROOT_ACCOUNT_TAG),
                seed_account("deriverseAuthority"),
            ]
        }),
    });
    m.push(InlinePda {
        instruction: "NewRootAccount",
        account: "community",
        pda: serde_json::json!({
            "seeds": [
                seed_arg("newRootAccountIdlArgs.version"),
                seed_const_u32(COMMUNITY_ACCOUNT_TAG),
                seed_account("deriverseAuthority"),
            ]
        }),
    });

    // clientPrimary — all instructions that touch it
    let client_primary_instructions: &[(&str, &str)] = &[
        ("Deposit",               "signer"),
        ("Withdraw",              "signer"),
        ("Airdrop",               "wallet"),
        ("AddWithdrawalAddress",  "signer"),
        ("VmChangeWhitelist",     "signer"),
        ("VmInitActivate",        "signer"),
        ("VmInitActivateCancel",  "signer"),
        ("VmFinalizeActivate",    "signer"),
        ("VmInitDeactivate",      "signer"),
        ("VmInitDeactivateCancel","signer"),
        ("VmFinalizeDeactivate",  "signer"),
        ("VmInitWithdraw",        "signer"),
        ("VmInitWithdrawCancel",  "signer"),
        ("VmInitWithdrawFinalize","signer"),
        ("VmDirectWithdraw",      "signer"),
        ("Voting",                "signer"),
        ("DividendsClaim",        "signer"),
        ("NewPrivateClient",      "wallet"),
    ];
    for (ix, wallet) in client_primary_instructions {
        m.push(InlinePda {
            instruction: ix,
            account: "clientPrimary",
            pda: serde_json::json!({
                "seeds": [
                    seed_account_typed("root.discriminator.version", "RootStateIdlAccount"),
                    seed_const_u32(CLIENT_PRIMARY_ACCOUNT_TAG),
                    seed_account(wallet),
                ]
            }),
        });
    }

    // clientCommunity
    for (ix, wallet) in [("Airdrop", "wallet"), ("Voting", "signer"), ("DividendsClaim", "signer")] {
        m.push(InlinePda {
            instruction: ix,
            account: "clientCommunity",
            pda: serde_json::json!({
                "seeds": [
                    seed_account_typed("root.discriminator.version", "RootStateIdlAccount"),
                    seed_const_u32(CLIENT_COMMUNITY_ACCOUNT_TAG),
                    seed_account(wallet),
                ]
            }),
        });
    }

    // clientVmAccount
    m.push(InlinePda {
        instruction: "AddWithdrawalAddress",
        account: "clientVmAccount",
        pda: serde_json::json!({
            "seeds": [
                seed_account_typed("root.discriminator.version", "RootStateIdlAccount"),
                seed_const_u32(VM_CLIENT_ACCOUNT_TAG),
                seed_account("signer"),
            ]
        }),
    });

    // Token accounts
    m.push(InlinePda {
        instruction: "Deposit",
        account: "clientTokenAccount",
        pda: serde_json::json!({
            "seeds": [seed_account("signer"), seed_account("tokenProgram"), seed_account("mint")],
            "program": seed_associated_token_program(),
        }),
    });
    for ix in ["Deposit", "Withdraw", "NewBaseCrncy", "WithdrawSwapFees", "VmInitWithdrawFinalize", "VmDirectWithdraw"] {
        m.push(InlinePda {
            instruction: ix,
            account: "programTokenAccount",
            pda: serde_json::json!({
                "seeds": [
                    seed_account("mint"),
                    seed_account_typed("root.discriminator.version", "RootStateIdlAccount"),
                ]
            }),
        });
    }
    m.push(InlinePda {
        instruction: "NewInstrument",
        account: "assetTokenProgramAccount",
        pda: serde_json::json!({
            "seeds": [
                seed_account("assetMint"),
                seed_account_typed("root.discriminator.version", "RootStateIdlAccount"),
            ]
        }),
    });
    m.push(InlinePda {
        instruction: "Airdrop",
        account: "authorityAssociatedTokenAccount",
        pda: serde_json::json!({
            "seeds": [seed_account("airdropAuthority"), seed_account("tokenProgram"), seed_account("drvsMint")],
            "program": seed_associated_token_program(),
        }),
    });
    m.push(InlinePda {
        instruction: "Airdrop",
        account: "drvsProgramTokenAccount",
        pda: serde_json::json!({
            "seeds": [
                seed_account("drvsMint"),
                seed_account_typed("root.discriminator.version", "RootStateIdlAccount"),
            ]
        }),
    });
    m.push(InlinePda {
        instruction: "Swap",
        account: "assetVaultTokenAccount",
        pda: serde_json::json!({
            "seeds": [
                seed_account("assetMint"),
                seed_account_typed("instrument.discriminator.version", "InstrAccountHeaderIdlAccount"),
            ]
        }),
    });
    m.push(InlinePda {
        instruction: "Swap",
        account: "currencyVaultTokenAccount",
        pda: serde_json::json!({
            "seeds": [
                seed_account("currencyMint"),
                seed_account_typed("instrument.discriminator.version", "InstrAccountHeaderIdlAccount"),
            ]
        }),
    });

    // NewInstrument — all spot sub-accounts
    let new_instr_accounts: &[(&str, u32)] = &[
        ("instrument",  INSTR_ACCOUNT_TAG),
        ("bidsTree",    SPOT_BIDS_TREE_ACCOUNT_TAG),
        ("asksTree",    SPOT_ASKS_TREE_ACCOUNT_TAG),
        ("bidOrders",   SPOT_BID_ORDERS_ACCOUNT_TAG),
        ("askOrders",   SPOT_ASK_ORDERS_ACCOUNT_TAG),
        ("lines",       SPOT_LINES_ACCOUNT_TAG),
        ("clientInfos", SPOT_CLIENT_INFOS_ACCOUNT_TAG),
    ];
    for (acc, tag) in new_instr_accounts {
        m.push(InlinePda {
            instruction: "NewInstrument",
            account: acc,
            pda: serde_json::json!({
                "seeds": [
                    seed_account_typed("root.discriminator.version", "RootStateIdlAccount"),
                    seed_const_u32(*tag),
                    seed_account_typed("assetToken.id", "TokenStateIdlAccount"),
                    seed_arg("newInstrumentIdlArgs.crncyTokenId"),
                    seed_account("deriverseAuthority"),
                ]
            }),
        });
    }
    m.push(InlinePda {
        instruction: "NewInstrument",
        account: "maps",
        pda: serde_json::json!({
            "derivation": "createWithSeed",
            "base": seed_account("signer"),
            "seeds": [
                seed_account_typed("root.discriminator.version", "RootStateIdlAccount"),
                seed_const_u32(SPOT_MAPS_ACCOUNT_TAG),
                seed_account_typed("assetToken.id", "TokenStateIdlAccount"),
                seed_arg("newInstrumentIdlArgs.crncyTokenId"),
            ]
        }),
    });

    // UpgradeToPerp — all perp sub-accounts
    let upgrade_accounts: &[(&str, u32)] = &[
        ("perpBidsTree",            PERP_BIDS_TREE_ACCOUNT_TAG),
        ("perpAsksTree",            PERP_ASKS_TREE_ACCOUNT_TAG),
        ("perpBidOrders",           PERP_BID_ORDERS_ACCOUNT_TAG),
        ("perpAskOrders",           PERP_ASK_ORDERS_ACCOUNT_TAG),
        ("perpLines",               PERP_LINES_ACCOUNT_TAG),
        ("perpClientInfos",         PERP_CLIENT_INFOS_ACCOUNT_TAG),
        ("perpClientInfos2",        PERP_CLIENT_INFOS2_ACCOUNT_TAG),
        ("perpClientInfos3",        PERP_CLIENT_INFOS3_ACCOUNT_TAG),
        ("perpClientInfos4",        PERP_CLIENT_INFOS4_ACCOUNT_TAG),
        ("perpClientInfos5",        PERP_CLIENT_INFOS5_ACCOUNT_TAG),
        ("perpLongPxTree",          PERP_LONG_PX_TREE_ACCOUNT_TAG),
        ("perpShortPxTree",         PERP_SHORT_PX_TREE_ACCOUNT_TAG),
        ("perpRebalanceTimeTree",   PERP_REBALANCE_TIME_TREE_ACCOUNT_TAG),
    ];
    for (acc, tag) in upgrade_accounts {
        m.push(InlinePda {
            instruction: "UpgradeToPerp",
            account: acc,
            pda: serde_json::json!({
                "seeds": [
                    seed_account_typed("root.discriminator.version", "RootStateIdlAccount"),
                    seed_const_u32(*tag),
                    seed_account_typed("instrument.assetTokenId", "InstrAccountHeaderIdlAccount"),
                    seed_account_typed("instrument.crncyTokenId", "InstrAccountHeaderIdlAccount"),
                    seed_account("deriverseAuthority"),
                ]
            }),
        });
    }
    m.push(InlinePda {
        instruction: "UpgradeToPerp",
        account: "perpMaps",
        pda: serde_json::json!({
            "derivation": "createWithSeed",
            "base": seed_account("signer"),
            "seeds": [
                seed_account_typed("root.discriminator.version", "RootStateIdlAccount"),
                seed_const_u32(PERP_MAPS_ACCOUNT_TAG),
                seed_account_typed("instrument.assetTokenId", "InstrAccountHeaderIdlAccount"),
                seed_account_typed("instrument.crncyTokenId", "InstrAccountHeaderIdlAccount"),
            ]
        }),
    });

    m
}

// ── Inject computed deriverseAuthority + tree seeds for trading instructions ──

/// Instructions that use spot tree accounts (bidsTree, asksTree, bidOrders, askOrders, lines, clientInfos)
/// but do NOT list deriverseAuthority as an explicit account.
const SPOT_TREE_INSTRUCTIONS: &[&str] = &[
    "NewSpotOrder", "SpotMassCancel", "SpotQuotesReplace",
];

/// Instructions that use perp tree accounts (perpBidsTree … perpRebalanceTimeTree)
/// but do NOT list deriverseAuthority as an explicit account.
const PERP_TREE_INSTRUCTIONS: &[&str] = &[
    "PerpDeposit", "NewPerpOrder", "PerpOrderCancel",
    "PerpMassCancel", "PerpChangeLeverage", "PerpStatisticsReset",
    "BuyMarketSeat", "SellMarketSeat", "PerpQuotesReplace",
];

fn inject_deriverse_authority_and_tree_seeds(idl: &mut serde_json::Value) {
    let da_pda = serde_json::json!({
        "name": "deriverseAuthority",
        "writable": false,
        "signer": false,
        "pda": { "seeds": [seed_const_ascii("ndxnt")] }
    });

    let spot_tree_accounts: &[(&str, u32)] = &[
        ("bidsTree",    SPOT_BIDS_TREE_ACCOUNT_TAG),
        ("asksTree",    SPOT_ASKS_TREE_ACCOUNT_TAG),
        ("bidOrders",   SPOT_BID_ORDERS_ACCOUNT_TAG),
        ("askOrders",   SPOT_ASK_ORDERS_ACCOUNT_TAG),
        ("lines",       SPOT_LINES_ACCOUNT_TAG),
        ("clientInfos", SPOT_CLIENT_INFOS_ACCOUNT_TAG),
    ];
    let perp_tree_accounts: &[(&str, u32)] = &[
        ("perpBidsTree",          PERP_BIDS_TREE_ACCOUNT_TAG),
        ("perpAsksTree",          PERP_ASKS_TREE_ACCOUNT_TAG),
        ("perpBidOrders",         PERP_BID_ORDERS_ACCOUNT_TAG),
        ("perpAskOrders",         PERP_ASK_ORDERS_ACCOUNT_TAG),
        ("perpLines",             PERP_LINES_ACCOUNT_TAG),
        ("perpClientInfos",       PERP_CLIENT_INFOS_ACCOUNT_TAG),
        ("perpClientInfos2",      PERP_CLIENT_INFOS2_ACCOUNT_TAG),
        ("perpClientInfos3",      PERP_CLIENT_INFOS3_ACCOUNT_TAG),
        ("perpClientInfos4",      PERP_CLIENT_INFOS4_ACCOUNT_TAG),
        ("perpClientInfos5",      PERP_CLIENT_INFOS5_ACCOUNT_TAG),
        ("perpLongPxTree",        PERP_LONG_PX_TREE_ACCOUNT_TAG),
        ("perpShortPxTree",       PERP_SHORT_PX_TREE_ACCOUNT_TAG),
        ("perpRebalanceTimeTree", PERP_REBALANCE_TIME_TREE_ACCOUNT_TAG),
    ];

    if let Some(instructions) = idl["instructions"].as_array_mut() {
        for ix in instructions.iter_mut() {
            let ix_name = ix["name"].as_str().unwrap_or("").to_string();

            let tree_accounts: &[(&str, u32)] =
                if SPOT_TREE_INSTRUCTIONS.iter().any(|n| n.eq_ignore_ascii_case(&ix_name)) {
                    spot_tree_accounts
                } else if PERP_TREE_INSTRUCTIONS.iter().any(|n| n.eq_ignore_ascii_case(&ix_name)) {
                    perp_tree_accounts
                } else {
                    continue;
                };

            let accounts = match ix["accounts"].as_array_mut() {
                Some(a) => a,
                None => continue,
            };

            // Inject deriverseAuthority computed account if not already present
            if !accounts.iter().any(|a| a["name"] == "deriverseAuthority") {
                accounts.push(da_pda.clone());
            }

            // Add seeds to each tree account
            for acc in accounts.iter_mut() {
                let name = acc["name"].as_str().unwrap_or("").to_string();
                if let Some((_, tag)) = tree_accounts.iter().find(|(n, _)| *n == name) {
                    if acc.get("pda").is_none() || acc["pda"].is_boolean() {
                        acc["pda"] = serde_json::json!({
                            "seeds": [
                                seed_account_typed("root.discriminator.version", "RootStateIdlAccount"),
                                seed_const_u32(*tag),
                                seed_account_typed("instrument.assetTokenId", "InstrAccountHeaderIdlAccount"),
                                seed_account_typed("instrument.crncyTokenId", "InstrAccountHeaderIdlAccount"),
                                seed_account("deriverseAuthority"),
                            ]
                        });
                    }
                }
            }
        }
    }
}

// ── Boolean pda marker (mirrors Python mark_instruction_account_pdas) ─────────

const TRUE_PDA_ACCOUNTS: &[&str] = &[
    "holderAccount", "root", "community", "privateClients",
    "clientPrimary", "clientCommunity", "clientVmAccount", "deriverseAuthority",
    "token", "assetToken", "currencyToken",
    "programTokenAccount", "assetTokenProgramAccount",
    "assetVaultTokenAccount", "currencyVaultTokenAccount", "drvsProgramTokenAccount",
    "instrument",
    "bidsTree", "asksTree", "bidOrders", "askOrders", "lines", "clientInfos",
    "perpBidsTree", "perpAsksTree", "perpBidOrders", "perpAskOrders",
    "perpLines", "perpClientInfos",
    "perpClientInfos2", "perpClientInfos3", "perpClientInfos4", "perpClientInfos5",
    "perpLongPxTree", "perpShortPxTree", "perpRebalanceTimeTree",
    "tree", "orders",
];

fn mark_pda_accounts(idl: &mut serde_json::Value) {
    let pda_set: std::collections::HashSet<&str> = TRUE_PDA_ACCOUNTS.iter().copied().collect();
    if let Some(instructions) = idl["instructions"].as_array_mut() {
        for ix in instructions.iter_mut() {
            if let Some(accounts) = ix["accounts"].as_array_mut() {
                for acc in accounts.iter_mut() {
                    if acc.get("pda").is_some() {
                        continue; // already has inline seeds, skip
                    }
                    let name = acc["name"].as_str().unwrap_or("").to_string();
                    let is_pda = pda_set.contains(name.as_str());
                    acc["pda"] = serde_json::json!(is_pda);
                }
            }
        }
    }
}

// ── Public entry point ────────────────────────────────────────────────────────

pub(crate) fn inject_pdas_into_idl(workspace_root: &Path, idl_path: &Path) {
    let types_rs_path = workspace_root.join("crates/models/src/state/types.rs");
    let pda_rs_path = workspace_root.join("crates/models/src/pda.rs");

    let types_src = std::fs::read_to_string(&types_rs_path).expect("failed to read types.rs");
    let types_file = syn::parse_file(&types_src).expect("failed to parse types.rs");

    let pda_src = std::fs::read_to_string(&pda_rs_path).expect("failed to read pda.rs");
    let pda_file = syn::parse_file(&pda_src).expect("failed to parse pda.rs");

    let consts = parse_account_type_constants(&types_file);
    let enum_names = parse_account_type_enum(&types_file);
    let entries = parse_drv_account_impls(&pda_file, &consts, &enum_names);

    let idl_src = std::fs::read_to_string(idl_path).expect("failed to read drv_models.json");
    let mut idl: serde_json::Value =
        serde_json::from_str(&idl_src).expect("failed to parse drv_models.json");

    // 1. Top-level pdas catalog (from impl DrvAccount for X blocks)
    let pdas: Vec<serde_json::Value> = entries
        .into_iter()
        .map(|e| serde_json::json!({ "name": e.name, "seeds": e.seeds }))
        .collect();
    idl["pdas"] = serde_json::json!(pdas);

    // 2. Inline PDA seeds on specific instruction accounts
    let mappings = inline_pda_mappings();
    for m in &mappings {
        let instructions = idl["instructions"].as_array_mut().unwrap();
        let ix = instructions.iter_mut().find(|ix| {
            ix["name"].as_str().map(|n| n.eq_ignore_ascii_case(m.instruction)).unwrap_or(false)
        });
        if let Some(ix) = ix {
            let accounts = ix["accounts"].as_array_mut().unwrap();
            if let Some(acc) = accounts.iter_mut().find(|a| a["name"] == m.account) {
                acc["pda"] = m.pda.clone();
            }
        }
    }

    // 3. Boolean pda marker for all remaining accounts
    mark_pda_accounts(&mut idl);

    // 4. Inject deriverseAuthority computed account + tree account seeds for trading instructions
    inject_deriverse_authority_and_tree_seeds(&mut idl);

    let out = serde_json::to_string_pretty(&idl).expect("failed to serialize IDL");
    std::fs::write(idl_path, out).expect("failed to write drv_models.json");
}
