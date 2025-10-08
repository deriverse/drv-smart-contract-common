use lazy_static::lazy_static;
use std::collections::HashMap;
use traits_implementations::get_implementation;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Attribute, Data, DeriveInput, Fields, Ident, Type};

use crate::errors::MacroError::{self, IncorrectInputType};

mod traits_implementations {
    use super::*;

    type ImplGenerator = fn(&Ident, &Type) -> TokenStream;

    fn impl_partial_eq(name: &Ident, field_type: &Type) -> TokenStream {
        quote! {
            impl PartialEq<#field_type> for #name {
                fn eq(&self, other: &#field_type) -> bool {
                    self.0 == *other
                }
            }

            impl PartialEq<#name> for #field_type {
                fn eq(&self, other: &#name) -> bool {
                    *self == other.0
                }
            }
        }
    }

    fn impl_partial_ord(name: &Ident, field_type: &Type) -> TokenStream {
        quote! {
            impl PartialOrd<#field_type> for #name {
                fn partial_cmp(&self, other: &#field_type) -> Option<std::cmp::Ordering> {
                    self.0.partial_cmp(other)
                }
            }

            impl PartialOrd<#name> for #field_type {
                fn partial_cmp(&self, other: &#name) -> Option<std::cmp::Ordering> {
                    self.partial_cmp(&other.0)
                }
            }
        }
    }

    fn create_impl_map() -> HashMap<&'static str, ImplGenerator> {
        let mut map = HashMap::new();

        let val: fn(&Ident, &Type) -> TokenStream = impl_partial_eq;
        map.insert("PartialEq", val);

        let val: fn(&Ident, &Type) -> TokenStream = impl_partial_ord;
        map.insert("PartialOrd", val);

        map
    }

    lazy_static! {
        pub static ref TRAIT_IMPLS: HashMap<&'static str, ImplGenerator> = create_impl_map();
    }

    pub(crate) fn get_implementation(
        trait_name: &str,
        name: &Ident,
        field_type: &Type,
    ) -> Option<TokenStream> {
        TRAIT_IMPLS
            .get(trait_name)
            .map(|generator| generator(name, field_type))
    }
}

/// As syn does not have standart parser for **derive**, custom "parser" is
/// needed
fn get_derive_traits(attr: &Attribute) -> Vec<String> {
    let mut trait_names = Vec::new();

    if let Ok(metas) = attr.parse_args_with(|input: syn::parse::ParseStream| {
        syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated(input)
    }) {
        for path in metas {
            if let Some(ident) = path.get_ident() {
                trait_names.push(ident.to_string());
            } else if !path.segments.is_empty() {
                let path_string = path
                    .segments
                    .iter()
                    .map(|seg| seg.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::");
                trait_names.push(path_string);
            }
        }
    }

    trait_names
}

pub(crate) fn new_type_inner(
    input: &DeriveInput,
    _: Vec<&Attribute>,
) -> Result<TokenStream, MacroError> {
    let name = &input.ident;

    let data = if let Data::Struct(ref data) = input.data {
        if let Fields::Unnamed(_) = data.fields {
            data
        } else {
            return Err(IncorrectInputType(
                "Field must be unnamed".to_string(),
                data.fields.span(),
            ));
        }
    } else {
        return Err(IncorrectInputType(
            "Must be struct".to_string(),
            input.span(),
        ));
    };

    let field_type = if let Some(field) = data.fields.iter().last() {
        &field.ty
    } else {
        return Err(IncorrectInputType(
            "Must have exactly 1 field".to_string(),
            data.fields.span(),
        ));
    };

    let addititional_impls: Vec<TokenStream> = input
        .attrs
        .iter()
        .find_map(|attr| {
            attr.path().get_ident().and_then(|ident| {
                if &*ident.to_string() == "derive" {
                    let traits = get_derive_traits(attr);

                    Some(
                        traits
                            .iter()
                            .filter_map(|traits_name| {
                                get_implementation(traits_name.as_str(), name, field_type)
                            })
                            .collect(),
                    )
                } else {
                    None
                }
            })
        })
        .unwrap_or(vec![quote! {}]);

    Ok(quote! {
        #input

        #(
            #addititional_impls
        )*

        impl #name {
            pub fn new(value: #field_type) -> Self {
                #name(value)
            }

            pub fn value(&self) -> u32 {
                self.0
            }
        }

        impl ::std::ops::Deref for #name {
            type Target = #field_type;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    })
}
