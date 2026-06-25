use std::path::Path;

fn is_internal_field(name: &str) -> bool {
    name == "tag" || name.starts_with("padding")
}

pub(crate) fn merge_instructions_into_idl(_workspace_root: &Path, idl_path: &Path) {
    let src = std::fs::read_to_string(idl_path).expect("failed to read drv_models.json");
    let mut idl: serde_json::Value =
        serde_json::from_str(&src).expect("failed to parse drv_models.json");

    // Strip tag + padding fields from every struct type definition
    if let Some(types) = idl["types"].as_array_mut() {
        for ty in types.iter_mut() {
            if let Some(fields) = ty["type"]["fields"].as_array_mut() {
                fields.retain(|f| {
                    f["name"]
                        .as_str()
                        .map(|n| !is_internal_field(n))
                        .unwrap_or(true)
                });
            }
        }
    }

    let out = serde_json::to_string_pretty(&idl).expect("failed to serialize IDL");
    std::fs::write(idl_path, out).expect("failed to write drv_models.json");
}
