use serde_json::Value;
use std::{collections::HashMap, fs, path::Path};

/// Primitive newtype wrappers and known semantic aliases to inline.
/// These are types that do not appear in the IDL types section (no ShankType
/// derive) but are referenced as {"defined": "..."} in fields.
const HARDCODED_ALIASES: &[(&str, &str)] = &[
    ("Version",        "u32"),
    ("Tag",            "u32"),
    ("InstrId",        "u32"),
    ("ClientId",       "u32"),
    ("InstrMask",      "u32"),
    ("InstrInputMask", "u8"),
    ("TokenMask",      "u32"),
    ("VmMask",         "u32"),
    ("QuoteMask",      "u16"),
    // Single-field struct wrappers that appear in the IDL types section:
    // CappedI64 { value: i64 } — semantically just a saturating i64
    ("CappedI64",      "i64"),
    // Rust f64 — shank has no f64 primitive so it emits {"defined":"f64"};
    // map to the "f64" string so Anchor v1 parsers can handle it.
    ("f64",            "f64"),
];

fn is_primitive_idl_type(ty: &Value) -> bool {
    matches!(
        ty.as_str(),
        Some(
            "u8" | "u16" | "u32" | "u64" | "u128"
            | "i8" | "i16" | "i32" | "i64" | "i128"
            | "bool" | "publicKey" | "pubkey" | "string" | "bytes"
        )
    )
}

/// Build the substitution map.
/// Only includes:
/// 1. The curated hardcoded aliases above.
/// 2. Newtype-style structs in the IDL types section: structs whose sole field
///    is unnamed (field name == "0") with a primitive type. These are emitted
///    by shank for `pub struct Foo(pub Bar)` patterns when Bar is a primitive.
fn build_substitution_map(types: &[Value]) -> HashMap<String, Value> {
    let mut map: HashMap<String, Value> = HashMap::new();

    for (name, prim) in HARDCODED_ALIASES {
        map.insert(name.to_string(), Value::String(prim.to_string()));
    }

    // Scan the IDL types section for unnamed single-field structs (newtypes).
    for type_def in types {
        let name = match type_def.get("name").and_then(|v| v.as_str()) {
            Some(n) => n.to_string(),
            None => continue,
        };
        if map.contains_key(&name) {
            continue;
        }
        let ty = match type_def.get("type") {
            Some(t) => t,
            None => continue,
        };
        if ty.get("kind").and_then(|v| v.as_str()) != Some("struct") {
            continue;
        }
        let fields = match ty.get("fields").and_then(|v| v.as_array()) {
            Some(f) if f.len() == 1 => f,
            _ => continue,
        };
        let field_name = fields[0].get("name").and_then(|v| v.as_str()).unwrap_or("");
        // Only inline true unnamed newtype wrappers (field "0")
        if field_name != "0" {
            continue;
        }
        let field_ty = match fields[0].get("type") {
            Some(t) => t.clone(),
            None => continue,
        };
        if is_primitive_idl_type(&field_ty) {
            map.insert(name, field_ty);
        }
    }

    map
}

/// Recursively walk a JSON value and replace every `{"defined": "X"}` object
/// where X is in the substitution map with the mapped type value.
fn substitute(value: &mut Value, map: &HashMap<String, Value>) {
    match value {
        Value::Object(obj) => {
            if let Some(defined_name) = obj
                .get("defined")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
            {
                if let Some(replacement) = map.get(&defined_name) {
                    *value = replacement.clone();
                    return;
                }
            }
            for v in obj.values_mut() {
                substitute(v, map);
            }
        }
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                substitute(v, map);
            }
        }
        _ => {}
    }
}

pub fn resolve_types(_root: &Path, idl_path: &Path) {
    let content = fs::read_to_string(idl_path).expect("Failed to read IDL");
    let mut idl: Value = serde_json::from_str(&content).expect("Failed to parse IDL");

    let types = idl
        .get("types")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let sub_map = build_substitution_map(&types);

    // Apply substitutions everywhere (including inside the types section so
    // e.g. Discriminator's fields get resolved: tag: Tag → tag: u32).
    substitute(&mut idl, &sub_map);

    // Remove from the types section types that were fully inlined.
    if let Some(Value::Array(types_arr)) = idl.get_mut("types") {
        types_arr.retain(|type_def| {
            let name = type_def
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            !sub_map.contains_key(name)
        });
    }

    let output =
        serde_json::to_string_pretty(&idl).expect("Failed to serialize IDL") + "\n";
    fs::write(idl_path, output).expect("Failed to write IDL");

    let mut resolved: Vec<&str> = sub_map.keys().map(|s| s.as_str()).collect();
    resolved.sort_unstable();
    println!("  Inlined {} defined types: {}", resolved.len(), resolved.join(", "));
}
