use serde_json::Value;
use std::{collections::HashMap, fs, path::Path};

use crate::solana_spec::apply_solana_spec_fixes;

fn remove_extensions(idl: &mut Value) {
    if let Some(obj) = idl.as_object_mut() {
        obj.remove("pdas");
    }
    if let Some(meta) = idl["metadata"].as_object_mut() {
        meta.remove("origin");
    }
}

fn remove_default_booleans(idl: &mut Value) {
    if let Some(instructions) = idl["instructions"].as_array_mut() {
        for ix in instructions.iter_mut() {
            if let Some(accounts) = ix["accounts"].as_array_mut() {
                for acc in accounts.iter_mut() {
                    if let Some(obj) = acc.as_object_mut() {
                        for key in ["writable", "signer", "optional"] {
                            if obj.get(key).and_then(|v| v.as_bool()) == Some(false) {
                                obj.remove(key);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn strip_attrs(value: &mut Value) {
    match value {
        Value::Object(obj) => {
            obj.remove("attrs");
            for v in obj.values_mut() {
                strip_attrs(v);
            }
        }
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                strip_attrs(v);
            }
        }
        _ => {}
    }
}

fn reorder_accounts(idl: &mut Value) {
    if let Some(accounts) = idl["accounts"].as_array_mut() {
        for acc in accounts.iter_mut() {
            if let Some(obj) = acc.as_object_mut() {
                if obj.contains_key("discriminator") && obj.contains_key("name") {
                    let disc = obj.remove("discriminator").unwrap();
                    obj.insert("discriminator".to_string(), disc);
                }
            }
        }
    }
}

/// Remove `docs` from every instruction account.
///
/// Solscan does not handle `docs` on instruction accounts and may reject the IDL.
fn strip_account_docs(idl: &mut Value) {
    if let Some(instructions) = idl["instructions"].as_array_mut() {
        for ix in instructions.iter_mut() {
            if let Some(accounts) = ix["accounts"].as_array_mut() {
                for acc in accounts.iter_mut() {
                    if let Some(obj) = acc.as_object_mut() {
                        obj.remove("docs");
                    }
                }
            }
        }
    }
}

/// Inline instruction data structs into `args`.
///
/// Shank wraps all instruction parameters in a single data struct and emits one arg
/// of type `{defined: {name: "XxxData"}}`.  Anchor-compatible consumers (Solscan)
/// expect scalar args listed directly.  This step replaces the single wrapper arg
/// with the fields of the referenced struct, taken from the `types` section.
///
/// The wrapper struct types themselves are kept in `types` so other references to
/// them (if any) still resolve.
fn inline_instruction_args(idl: &mut Value) {
    // Build a map of type-name → fields from the types section.
    let type_fields: HashMap<String, Vec<Value>> = idl["types"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|t| {
                    let name = t["name"].as_str()?.to_string();
                    let fields = t["type"]["fields"].as_array()?.clone();
                    Some((name, fields))
                })
                .collect()
        })
        .unwrap_or_default();

    if let Some(instructions) = idl["instructions"].as_array_mut() {
        for ix in instructions.iter_mut() {
            let args = match ix["args"].as_array() {
                Some(a) => a.clone(),
                None => continue,
            };
            // Only expand when there is exactly one arg whose type is a defined struct.
            if args.len() != 1 {
                continue;
            }
            let arg = &args[0];
            let type_name = match arg["type"].get("defined").and_then(|d| d["name"].as_str()) {
                Some(n) => n.to_string(),
                None => continue,
            };
            if let Some(fields) = type_fields.get(&type_name) {
                ix["args"] = Value::Array(fields.clone());
            }
        }
    }
}

pub fn align_to_anchor_spec(_root: &Path, idl_path: &Path) {
    let content = fs::read_to_string(idl_path).expect("Failed to read IDL");
    let mut idl: Value = serde_json::from_str(&content).expect("Failed to parse IDL");

    apply_solana_spec_fixes(&mut idl);

    println!("  [8] Removing Shank extension fields (pdas, metadata.origin)...");
    remove_extensions(&mut idl);

    println!("  [9] Removing default false booleans from instruction accounts...");
    remove_default_booleans(&mut idl);

    println!(" [10] Stripping attrs from all fields...");
    strip_attrs(&mut idl);

    println!(" [11] Reordering account entries (name before discriminator)...");
    reorder_accounts(&mut idl);

    println!(" [12] Stripping docs from instruction accounts...");
    strip_account_docs(&mut idl);

    println!(" [13] Inlining instruction data structs into args...");
    inline_instruction_args(&mut idl);

    let output = serde_json::to_string_pretty(&idl).expect("Failed to serialize IDL") + "\n";
    fs::write(idl_path, output).expect("Failed to write IDL");
}
