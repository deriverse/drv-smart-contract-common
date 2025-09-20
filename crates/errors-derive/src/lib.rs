use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Lit};

#[proc_macro_derive(DrvError, attributes(error))]
pub fn solana_error_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("DrvError can only be derived for enums"),
    };

    // Collect error metadata (same as before)
    let mut error_meta = Vec::new();

    for variant in variants {
        let variant_name = &variant.ident;
        let mut code = None;
        let mut msg = None;

        for attr in &variant.attrs {
            if attr.path().is_ident("error") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("code") {
                        let value = meta.value()?;
                        let lit: Lit = value.parse()?;
                        if let Lit::Int(lit_int) = lit {
                            code = Some(lit_int.base10_parse::<u32>().unwrap());
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
                .unwrap();
            }
        }

        let code = code.expect(&format!(
            "Missing 'code' attribute for variant {}",
            variant_name
        ));
        let msg = msg.expect(&format!(
            "Missing 'msg' attribute for variant {}",
            variant_name
        ));

        error_meta.push((variant_name, code, msg, variant.fields.clone()));
    }

    // Generate code() method
    let code_arms = error_meta.iter().map(|(variant_name, code, _, fields)| {
        let pattern = match fields {
            Fields::Unit => quote! { #name::#variant_name },
            Fields::Named(_) => quote! { #name::#variant_name { .. } },
            Fields::Unnamed(_) => quote! { #name::#variant_name(..) },
        };
        quote! { #pattern => #code }
    });

    // Generate to_json() method
    let json_arms = error_meta
        .iter()
        .map(|(variant_name, code, msg, fields)| match fields {
            Fields::Unit => {
                quote! {
                    #name::#variant_name => {
                        serde_json::json!({
                            "code": #code,
                            "msg": #msg,
                        })
                    }
                }
            }
            Fields::Named(fields_named) => {
                let field_names: Vec<_> = fields_named
                    .named
                    .iter()
                    .map(|f| f.ident.as_ref().unwrap())
                    .collect();

                let mut msg_format_str = msg.clone();
                let mut format_args = Vec::new();

                for field_name in &field_names {
                    let field_str = field_name.to_string();
                    let placeholder = format!("{{{}}}", field_str);
                    if msg_format_str.contains(&placeholder) {
                        msg_format_str = msg_format_str.replace(&placeholder, "{}");
                        format_args.push(field_name);
                    }
                }

                let json_fields = field_names.iter().map(|field_name| {
                    let field_str = field_name.to_string();
                    let needs_to_string = field_str.contains("account")
                        || field_str.contains("address")
                        || field_str.contains("pubkey")
                        || field_str.contains("key")
                        || field_str.contains("owner")
                        || field_str.contains("authority");

                    if needs_to_string {
                        quote! { #field_str: #field_name.to_string() }
                    } else {
                        quote! { #field_str: #field_name }
                    }
                });

                let msg_generation = if format_args.is_empty() {
                    quote! { #msg_format_str.to_string() }
                } else {
                    quote! { format!(#msg_format_str, #(#format_args),*) }
                };

                quote! {
                    #name::#variant_name { #(#field_names),* } => {
                        serde_json::json!({
                            "code": #code,
                            "msg": #msg_generation,
                            #(#json_fields),*
                        })
                    }
                }
            }
            Fields::Unnamed(_) => {
                panic!("Unnamed fields are not supported for DrvError");
            }
        });

    // Generate Display implementation
    let display_arms = error_meta
        .iter()
        .map(|(variant_name, _, msg, fields)| match fields {
            Fields::Unit => {
                quote! {
                    #name::#variant_name => write!(f, #msg)
                }
            }
            Fields::Named(fields_named) => {
                let field_names: Vec<_> = fields_named
                    .named
                    .iter()
                    .map(|f| f.ident.as_ref().unwrap())
                    .collect();

                let mut msg_format_str = msg.clone();
                let mut format_args = Vec::new();

                for field_name in &field_names {
                    let field_str = field_name.to_string();
                    let placeholder = format!("{{{}}}", field_str);
                    if msg_format_str.contains(&placeholder) {
                        msg_format_str = msg_format_str.replace(&placeholder, "{}");
                        format_args.push(field_name);
                    }
                }

                if format_args.is_empty() {
                    quote! {
                        #name::#variant_name { .. } => write!(f, #msg_format_str)
                    }
                } else {
                    quote! {
                        #name::#variant_name { #(#field_names),* } => {
                            write!(f, #msg_format_str, #(#format_args),*)
                        }
                    }
                }
            }
            Fields::Unnamed(_) => {
                panic!("Unnamed fields are not supported for DrvError");
            }
        });

    let expanded = quote! {
        impl #name {
            pub fn code(&self) -> u32 {
                match self {
                    #(#code_arms),*
                }
            }

            pub fn to_json(&self) -> serde_json::Value {
                match self {
                    #(#json_arms),*
                }
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display_arms),*
                }
            }
        }

        impl std::error::Error for #name {}
    };

    TokenStream::from(expanded)
}
