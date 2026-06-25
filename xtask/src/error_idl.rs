use std::path::Path;

use syn::{Item, Lit};

pub(crate) fn generate_error_idl(workspace_root: &Path) -> Vec<serde_json::Value> {
    let src = std::fs::read_to_string(
        workspace_root.join("crates/errors/src/errors.rs"),
    )
    .expect("failed to read errors.rs");

    let file = syn::parse_file(&src).expect("failed to parse errors.rs");

    let error_enum = file
        .items
        .iter()
        .find_map(|item| match item {
            Item::Enum(e) if e.ident == "DeriverseErrorKind" => Some(e),
            _ => None,
        })
        .expect("DeriverseErrorKind enum not found in errors.rs");

    error_enum
        .variants
        .iter()
        .map(|variant| {
            let name = variant.ident.to_string();
            let mut code: Option<u32> = None;
            let mut msg: Option<String> = None;

            for attr in &variant.attrs {
                if attr.path().is_ident("error") {
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("code") {
                            let value = meta.value()?;
                            let lit: Lit = value.parse()?;
                            if let Lit::Int(lit_int) = lit {
                                code = Some(lit_int.base10_parse().expect("code must be u32"));
                            }
                        } else if meta.path.is_ident("msg") {
                            let value = meta.value()?;
                            let lit: Lit = value.parse()?;
                            if let Lit::Str(lit_str) = lit {
                                msg = Some(lit_str.value());
                            }
                        }
                        Ok(())
                    })
                    .expect("failed to parse #[error(...)] attribute");
                }
            }

            let code = code.unwrap_or_else(|| panic!("variant {name} is missing code in #[error]"));
            let msg = msg.unwrap_or_else(|| panic!("variant {name} is missing msg in #[error]"));

            serde_json::json!({ "code": code, "name": name, "msg": msg })
        })
        .collect()
}

pub(crate) fn inject_errors_into_idl(_workspace_root: &Path, errors: Vec<serde_json::Value>, idl_path: &Path) {
    let src = std::fs::read_to_string(idl_path).expect("failed to read drv_models.json");
    let mut idl: serde_json::Value = serde_json::from_str(&src).expect("failed to parse drv_models.json");

    idl["errors"] = serde_json::Value::Array(errors);

    let out = serde_json::to_string_pretty(&idl).expect("failed to serialize IDL");
    std::fs::write(idl_path, out).expect("failed to write drv_models.json");
}
