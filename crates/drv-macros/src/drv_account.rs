use std::collections::HashSet;

use proc_macro2::TokenStream;

use crate::errors::MacroError;
use quote::{format_ident, quote};
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Attribute, Data, DeriveInput, Field,
    Fields, Generics, Ident, Type, Visibility,
};

/// As macros add an inner struct, initialization with struct literal become
/// complex. And a **new** constructor is needed
fn new_function(
    fields: &Punctuated<Field, Comma>,
    shadow_name: &Ident,
) -> Result<TokenStream, MacroError> {
    let mut names: Vec<Ident> = vec![];

    let input_args = fields.iter().map(|field| {
        // never panics
        let field_name = field.ident.clone().unwrap();
        names.push(field_name.clone());
        let field_type = field.ty.clone();
        quote! {
            #field_name: #field_type
        }
    });
    let args = quote! {#(#input_args), *};

    let inner = quote! {
        Self {
            inner: #shadow_name {
                #(#names), *
            }
        }
    };

    Ok(quote! {
        pub fn new(#args) -> Self {
            #inner
        }
    })
}

/// Function which is used to detect generics in the type.
///
/// Currently, function is recurrent and does not handle complex types.
/// Still this should be enough for the system as we avoid complexity.
fn check_type_for_generic(field_type: &Type, generics: &HashSet<String>) -> Result<(), MacroError> {
    match field_type {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.first() {
                let ident = segment.ident.to_string();
                if generics.contains(&ident) {
                    return Err(MacroError::UsedGeneric(ident, field_type.span()));
                }
            }

            for segment in &type_path.path.segments {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    for arg in &args.args {
                        if let syn::GenericArgument::Type(inner_type) = arg {
                            check_type_for_generic(inner_type, generics)?;
                        }
                    }
                }
            }
        }
        Type::Reference(type_ref) => {
            check_type_for_generic(&type_ref.elem, generics)?;
        }
        Type::Array(type_array) => {
            check_type_for_generic(&type_array.elem, generics)?;
        }
        Type::Tuple(type_tuple) => {
            for elem in &type_tuple.elems {
                check_type_for_generic(elem, generics)?;
            }
        }

        _ => {}
    }

    Ok(())
}

fn create_wrapper(
    vis: &Visibility,
    name: &Ident,
    inner_name: &Ident,
    attrs: &Vec<&Attribute>,
    fields: &Punctuated<Field, Comma>,
    generics: &Generics,
) -> Result<TokenStream, MacroError> {
    let generics_named: HashSet<String> = generics
        .params
        .iter()
        .map(|generic| {
            let name = match generic {
                syn::GenericParam::Lifetime(lifetime_param) => {
                    lifetime_param.lifetime.ident.to_string()
                }

                syn::GenericParam::Type(type_param) => type_param.ident.to_string(),
                syn::GenericParam::Const(const_param) => const_param.ident.to_string(),
            };

            name
        })
        .collect();

    fields.iter().try_for_each(|field| {
        let field_type = &field.ty;
        check_type_for_generic(field_type, &generics_named)
    })?;

    Ok(quote! {
        #(#attrs)*
        #vis struct #inner_name {
            #fields
        }

        #[repr(transparent)]
        #[derive(Copy, Clone, Pod, Zeroable)]
        #vis struct #name #generics {
            pub inner: #inner_name,
        }
    })
}

fn impl_derefs(name: &Ident, inner_name: &Ident, generics: &Generics) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics std::ops::Deref for #name #ty_generics #where_clause {
            type Target = #inner_name;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl #impl_generics std::ops::DerefMut for #name #ty_generics #where_clause {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

    }
}
#[inline]
pub(crate) fn drv_account_inner(
    input: &DeriveInput,
    attrs: Vec<&Attribute>,
) -> Result<TokenStream, MacroError> {
    let name = &input.ident;
    let vis = &input.vis;

    let fields = if let Data::Struct(data) = &input.data {
        match &data.fields {
            Fields::Named(fields_named) => Ok(&fields_named.named),
            _ => return Err(MacroError::UnnamedField(data.fields.span().clone())),
        }
    } else {
        Err(MacroError::IncorrectEntityType(
            "Only structs are supported".to_string(),
            input.span(),
        ))
    }?;

    let generics = &input.generics;

    let shadow_name: proc_macro2::Ident = format_ident!("{}NonGen", name);

    let account_structs = if generics.params.len() > 0 {
        let wrapper = create_wrapper(&vis, name, &shadow_name, &attrs, fields, generics);
        if let Err(err) = wrapper {
            Err(err)
        } else {
            Ok(wrapper.unwrap())
        }
    } else {
        Err(MacroError::IncorrectMacroUsage(
            "Struct must have at least one generic".to_string(),
            generics.span(),
        ))
    }?;

    let new_method = new_function(fields, &shadow_name)?;

    let derefs = impl_derefs(name, &shadow_name, generics);

    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    Ok(quote! {
        #account_structs

        impl #impl_generics #name #ty_generics  {
            #new_method
        }

        #derefs
    })
}
