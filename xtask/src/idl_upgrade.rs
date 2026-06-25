use serde_json::Value;
use std::{fs, path::Path};

/// Recursively rename `"publicKey"` → `"pubkey"` in every type position.
/// Anchor IDL v0 (shank) uses camelCase `"publicKey"`; v1 uses lowercase `"pubkey"`.
fn rename_public_key(value: &mut Value) {
    match value {
        Value::String(s) if s == "publicKey" => *s = "pubkey".to_string(),
        Value::Object(obj) => {
            for v in obj.values_mut() {
                rename_public_key(v);
            }
        }
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                rename_public_key(v);
            }
        }
        _ => {}
    }
}

/// Upgrade the IDL from Anchor v0 (shank) format to Anchor v1 format so that
/// tools like Codama and Solscan can parse PDA seeds on instruction accounts.
///
/// Changes applied:
/// - `metadata.spec` set to `"0.1.0"`
/// - `isMut` → `writable` on every instruction account
/// - `isSigner` → `signer` on every instruction account
/// - `"publicKey"` → `"pubkey"` everywhere (Anchor v1 type name)
pub fn upgrade_to_v1(_root: &Path, idl_path: &Path) {
    let content = fs::read_to_string(idl_path).expect("Failed to read IDL");
    let mut idl: Value = serde_json::from_str(&content).expect("Failed to parse IDL");

    // Set spec version
    idl["metadata"]["spec"] = Value::String("0.1.0".to_string());

    // Rename isMut → writable, isSigner → signer on instruction accounts
    if let Some(instructions) = idl["instructions"].as_array_mut() {
        for ix in instructions.iter_mut() {
            if let Some(accounts) = ix["accounts"].as_array_mut() {
                for acc in accounts.iter_mut() {
                    if let Some(obj) = acc.as_object_mut() {
                        if let Some(v) = obj.remove("isMut") {
                            obj.insert("writable".to_string(), v);
                        }
                        if let Some(v) = obj.remove("isSigner") {
                            obj.insert("signer".to_string(), v);
                        }
                    }
                }
            }
        }
    }

    // Rename publicKey → pubkey (Anchor v0 → v1 type name)
    rename_public_key(&mut idl);

    let output = serde_json::to_string_pretty(&idl).expect("Failed to serialize IDL") + "\n";
    fs::write(idl_path, output).expect("Failed to write IDL");
}
