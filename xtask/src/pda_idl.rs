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

    // Top-level pdas catalog only — instruction account seeds come from Shank annotations.
    let pdas: Vec<serde_json::Value> = entries
        .into_iter()
        .map(|e| serde_json::json!({ "name": e.name, "seeds": e.seeds }))
        .collect();
    idl["pdas"] = serde_json::json!(pdas);

    let out = serde_json::to_string_pretty(&idl).expect("failed to serialize IDL");
    std::fs::write(idl_path, out).expect("failed to write drv_models.json");
}
