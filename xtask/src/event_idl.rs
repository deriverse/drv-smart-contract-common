use std::{collections::HashMap, path::Path};

/// Parse `pub const X: u8 = N;` items from the `log_type` submodule.
fn parse_log_type_constants(file: &syn::File) -> HashMap<String, u8> {
    for item in &file.items {
        if let syn::Item::Mod(m) = item {
            if m.ident == "log_type" {
                if let Some((_, items)) = &m.content {
                    return items
                        .iter()
                        .filter_map(|item| {
                            let c = match item {
                                syn::Item::Const(c) => c,
                                _ => return None,
                            };
                            let name = c.ident.to_string();
                            let value = match c.expr.as_ref() {
                                syn::Expr::Lit(e) => match &e.lit {
                                    syn::Lit::Int(n) => n.base10_parse::<u8>().ok()?,
                                    _ => return None,
                                },
                                _ => return None,
                            };
                            Some((name, value))
                        })
                        .collect();
                }
            }
        }
    }
    HashMap::new()
}

/// Returns Vec<(struct_name, tag_const_name)>.
fn parse_log_impls(file: &syn::File) -> Vec<(String, String)> {
    file.items
        .iter()
        .filter_map(|item| {
            let impl_block = match item {
                syn::Item::Impl(i) => i,
                _ => return None,
            };
            // Must be `impl Log for X`
            let (_, trait_path, _) = impl_block.trait_.as_ref()?;
            if trait_path.segments.last()?.ident != "Log" {
                return None;
            }
            let struct_name = match impl_block.self_ty.as_ref() {
                syn::Type::Path(tp) => tp.path.segments.last()?.ident.to_string(),
                _ => return None,
            };
            // Find `const TAG: u8 = CONST_NAME;`
            for impl_item in &impl_block.items {
                if let syn::ImplItem::Const(c) = impl_item {
                    if c.ident != "TAG" {
                        continue;
                    }
                    let const_name = match &c.expr {
                        syn::Expr::Path(ep) => ep.path.segments.last()?.ident.to_string(),
                        _ => continue,
                    };
                    return Some((struct_name, const_name));
                }
            }
            None
        })
        .collect()
}

fn is_internal_field(name: &str) -> bool {
    name == "tag" || name.starts_with("padding")
}

pub(crate) fn inject_events_into_idl(workspace_root: &Path, idl_path: &Path) {
    let log_rs_path = workspace_root.join("crates/models/src/log.rs");

    let log_src = std::fs::read_to_string(&log_rs_path).expect("failed to read log.rs");
    let log_file = syn::parse_file(&log_src).expect("failed to parse log.rs");

    let constants = parse_log_type_constants(&log_file);
    let log_impls = parse_log_impls(&log_file);

    let idl_src = std::fs::read_to_string(idl_path).expect("failed to read drv_models.json");
    let mut idl: serde_json::Value =
        serde_json::from_str(&idl_src).expect("failed to parse drv_models.json");

    // Build lookup: struct name → fields (as shank emitted them)
    let types_by_name: HashMap<String, serde_json::Value> = match idl["types"].as_array() {
        None => HashMap::new(),
        Some(arr) => arr
            .iter()
            .filter_map(|t| {
                let name = t["name"].as_str()?.to_string();
                Some((name, t["type"]["fields"].clone()))
            })
            .collect(),
    };

    let mut events: Vec<serde_json::Value> = log_impls
        .into_iter()
        .filter_map(|(struct_name, const_name)| {
            let discriminant = match constants.get(&const_name) {
                Some(&v) => v,
                None => {
                    eprintln!(
                        "warn: constant {const_name} not found in log_type, skipping {struct_name}"
                    );
                    return None;
                }
            };
            let raw_fields = match types_by_name.get(&struct_name) {
                Some(f) => f,
                None => {
                    eprintln!("warn: no IDL type for {struct_name}, skipping");
                    return None;
                }
            };

            let fields: Vec<serde_json::Value> = raw_fields
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter(|f| {
                    f["name"]
                        .as_str()
                        .map(|n| !is_internal_field(n))
                        .unwrap_or(true)
                })
                .cloned()
                .collect();

            Some(serde_json::json!({
                "name": struct_name,
                "discriminant": { "type": "u8", "value": discriminant },
                "fields": fields
            }))
        })
        .collect();

    events.sort_by_key(|e| e["discriminant"]["value"].as_u64().unwrap_or(0));

    idl["events"] = serde_json::json!(events);

    let out = serde_json::to_string_pretty(&idl).expect("failed to serialize IDL");
    std::fs::write(idl_path, out).expect("failed to write drv_models.json");
}
