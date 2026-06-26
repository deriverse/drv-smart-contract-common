use serde_json::{json, Value};
use std::{collections::HashSet, fs, path::Path};

/// Fix 3: Move `metadata.address` → root `address`, root `name`/`version` → `metadata`.
fn fix_root_structure(idl: &mut Value) {
    let address = idl["metadata"]
        .get("address")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let name = idl
        .get("name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let version = idl
        .get("version")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    if let Some(addr) = address {
        idl["address"] = Value::String(addr);
        idl["metadata"].as_object_mut().unwrap().remove("address");
    }
    if let Some(n) = name {
        idl["metadata"]["name"] = Value::String(n);
        idl.as_object_mut().unwrap().remove("name");
    }
    if let Some(v) = version {
        idl["metadata"]["version"] = Value::String(v);
        idl.as_object_mut().unwrap().remove("version");
    }
}

/// Fix 2: `"discriminant": {"type": "u8", "value": N}` → `"discriminator": [N]`
fn fix_instruction_discriminators(idl: &mut Value) {
    if let Some(instructions) = idl["instructions"].as_array_mut() {
        for ix in instructions.iter_mut() {
            if let Some(obj) = ix.as_object_mut() {
                if let Some(discriminant) = obj.remove("discriminant") {
                    let value = discriminant["value"].as_u64().unwrap_or(0) as u8;
                    obj.insert("discriminator".to_string(), json!([value]));
                }
            }
        }
    }
}

/// Fix 4: Remove `"pda": false` from instruction accounts (absent means non-PDA per spec).
fn remove_false_pda_markers(idl: &mut Value) {
    if let Some(instructions) = idl["instructions"].as_array_mut() {
        for ix in instructions.iter_mut() {
            if let Some(accounts) = ix["accounts"].as_array_mut() {
                for acc in accounts.iter_mut() {
                    if let Some(obj) = acc.as_object_mut() {
                        if obj.get("pda").and_then(|v| v.as_bool()) == Some(false) {
                            obj.remove("pda");
                        }
                    }
                }
            }
        }
    }
}

/// Fix 5: Convert `kind: "param"` seeds to `kind: "arg"` in the top-level `pdas` catalog.
///
/// Shank emits `{"kind": "param", "name": "x", "type": "pubkey"}`; spec requires
/// `{"kind": "arg", "path": "x"}`.
fn fix_pda_seed_kinds(idl: &mut Value) {
    if let Some(pdas) = idl["pdas"].as_array_mut() {
        for pda in pdas.iter_mut() {
            if let Some(seeds) = pda["seeds"].as_array_mut() {
                for seed in seeds.iter_mut() {
                    if let Some(obj) = seed.as_object_mut() {
                        if obj.get("kind").and_then(|v| v.as_str()) == Some("param") {
                            obj.insert("kind".to_string(), Value::String("arg".to_string()));
                            if let Some(name_val) = obj.remove("name") {
                                obj.insert("path".to_string(), name_val);
                            }
                            obj.remove("type");
                        }
                    }
                }
            }
        }
    }
}

/// Fix 6: Move inline account type definitions into the `types` section.
///
/// The spec requires `accounts` entries to be reference-only (`name` + `discriminator`).
/// Type details belong in `types`. The inline `"type": {...}` is lifted out and appended
/// to `types`, skipping any name that already exists there.
fn extract_account_types(idl: &mut Value) {
    let existing_names: HashSet<String> = idl["types"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|t| t["name"].as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    let mut extracted: Vec<Value> = Vec::new();

    if let Some(accounts) = idl["accounts"].as_array_mut() {
        for acc in accounts.iter_mut() {
            let name = acc["name"].as_str().map(|s| s.to_string());
            if let Some(name) = name {
                if !existing_names.contains(&name) {
                    if let Some(obj) = acc.as_object_mut() {
                        if let Some(type_def) = obj.remove("type") {
                            extracted.push(json!({ "name": name, "type": type_def }));
                        }
                    }
                } else {
                    // Already in types — just drop the inline definition.
                    if let Some(obj) = acc.as_object_mut() {
                        obj.remove("type");
                    }
                }
            }
        }
    }

    if let Some(types_arr) = idl["types"].as_array_mut() {
        for t in extracted {
            types_arr.push(t);
        }
    }
}

/// Fix 1: `{"defined": "TypeName"}` → `{"defined": {"name": "TypeName"}}` everywhere.
///
/// Must run after `resolve_types` so that primitive aliases (`Tag`, `Version`, etc.)
/// have already been inlined and are no longer present as `defined` references.
fn fix_defined_type_refs(value: &mut Value) {
    match value {
        Value::Object(obj) => {
            // If this object is exactly a `defined` reference with a string value, upgrade it.
            if let Some(s) = obj.get("defined").and_then(|v| v.as_str()).map(|s| s.to_string()) {
                obj.insert("defined".to_string(), json!({ "name": s }));
                return; // no further nesting inside a bare defined-reference
            }
            for v in obj.values_mut() {
                fix_defined_type_refs(v);
            }
        }
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                fix_defined_type_refs(v);
            }
        }
        _ => {}
    }
}

/// Account name → TAG value, mirroring `DrvAccount::TAG` impls in `pda.rs`.
///
/// The discriminator written into the IDL is the 4-byte little-endian encoding of the tag.
/// These are the first 4 bytes of every on-chain account (the `tag` field of `Discriminator`).
const ACCOUNT_TAGS: &[(&str, u32)] = &[
    ("ClientCommunityAccountHeader", 35), // account_type::CLIENT_COMMUNITY
    ("ClientPrimaryAccountHeader",   31), // account_type::CLIENT_PRIMARY
    ("CommunityAccountHeader",       34), // account_type::COMMUNITY
    ("HolderAccountHeader",           1), // account_type::HOLDER
    ("InstrAccountHeader",            7), // account_type::INSTR
    ("PrivateClientHeader",          51), // account_type::PRIVATE_CLIENTS
    ("RootState",                     2), // account_type::ROOT
    ("TokenState",                    4), // account_type::TOKEN
    ("ClientVmAccountHeader",        52), // account_type::VM_CLIENT
];

/// Fix 7: Inject `"discriminator": [b0, b1, b2, b3]` (LE u32 tag) into each account entry.
fn inject_account_discriminators(idl: &mut Value) {
    if let Some(accounts) = idl["accounts"].as_array_mut() {
        for acc in accounts.iter_mut() {
            let name = match acc["name"].as_str() {
                Some(n) => n.to_string(),
                None => continue,
            };
            if let Some((_, tag)) = ACCOUNT_TAGS.iter().find(|(n, _)| *n == name) {
                let bytes: Vec<u8> = tag.to_le_bytes().to_vec();
                acc.as_object_mut()
                    .unwrap()
                    .insert("discriminator".to_string(), json!(bytes));
            }
        }
    }
}

/// Apply all Solana IDL spec v0.1.0 fixes to an in-memory IDL value.
pub(crate) fn apply_solana_spec_fixes(idl: &mut Value) {
    println!("  [1] Converting defined type references to object form...");
    fix_defined_type_refs(idl);

    println!("  [2] Converting instruction discriminants to byte-array form...");
    fix_instruction_discriminators(idl);

    println!("  [3] Fixing root structure (address / metadata.name / metadata.version)...");
    fix_root_structure(idl);

    println!("  [4] Removing pda:false markers from instruction accounts...");
    remove_false_pda_markers(idl);

    println!("  [5] Converting PDA seed kind:param → kind:arg in top-level pdas...");
    fix_pda_seed_kinds(idl);

    println!("  [6] Extracting account type definitions into the types section...");
    extract_account_types(idl);

    println!("  [7] Injecting account discriminators (LE u32 tag bytes)...");
    inject_account_discriminators(idl);
}

pub fn align_to_solana_spec(_root: &Path, idl_path: &Path) {
    let content = fs::read_to_string(idl_path).expect("Failed to read IDL");
    let mut idl: Value = serde_json::from_str(&content).expect("Failed to parse IDL");

    apply_solana_spec_fixes(&mut idl);

    let output = serde_json::to_string_pretty(&idl).expect("Failed to serialize IDL") + "\n";
    fs::write(idl_path, output).expect("Failed to write IDL");
}
