use drv_account::drv_account_inner;
use new_type::new_type_inner;
use syn::{parse_macro_input, Attribute, DeriveInput, ItemFn};

use proc_macro::TokenStream;

use cu_stats::cu_stats_inner;
mod cu_stats;
mod drv_account;
mod errors;
mod new_type;

#[proc_macro_attribute]
/// ## Workaround for `bytemuck` generic types problem:
/// Pod crate currently does not allow definition of structs with generics,
/// which do not impact structs memory layout.
///
/// ```ignore
/// struct TestStruct<const IDENT: u32> {
///     size: u32,
///     index: u32
/// }
/// ```
///
/// ## Requirements:
/// 1. Must be #[repr(C)]
/// 2. Must implement Pod (Pod, Zeroable, Clone, Copy)
/// 3. Must have al least 1 generic parameter
///
/// ## Macro output:
///
/// This macro provides a solution for Pods uncertainty about generics
/// which do not affect memory layout of the struct. When generics are detected:
///
/// 1. Create a non-generic inner structure with the actual fields.
/// 2. Wraps it in a transparent generic structure
/// 3. Implements necessary trait for seamless usage
///
/// In case of existence of generic affecting memory layout for relatively
/// simple types helpfull compile time error is generated.

pub fn pod_wrapper(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let attrs: Vec<&Attribute> = input
        .attrs
        .iter()
        .filter(|attr| !attr.path().is_ident("pod_wrapper"))
        .collect();

    let result = drv_account_inner(&input, attrs)
        .unwrap_or_else(|err| err.to_syn_error().to_compile_error().into());

    proc_macro::TokenStream::from(result)
}

#[proc_macro_attribute]
/// ## New type macro
/// Macro is used to clean up the code of new types creation.
///
/// As we can not define a single group of traits which would fit for every
/// **new type** and code size is important, macro look through **derive**
/// attribute and find trait which can be used in new type patern.
///
/// Etc. PartialEq, PartialOrd
pub fn new_type(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let attrs: Vec<&Attribute> = input
        .attrs
        .iter()
        .filter(|attr| !attr.path().is_ident("new_type"))
        .collect();

    let result = new_type_inner(&input, attrs)
        .unwrap_or_else(|err| err.to_syn_error().to_compile_error().into());

    TokenStream::from(result)
}

#[proc_macro_attribute]
/// ## Cu stats macro
/// Macro is used for instructions spedning CU calculations
pub fn cu_stats(
    _attrs: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    proc_macro::TokenStream::from(cu_stats_inner(input_fn))
}
