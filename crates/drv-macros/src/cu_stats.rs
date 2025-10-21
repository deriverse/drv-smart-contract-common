use proc_macro2::TokenStream;
use quote::quote;

use syn::{parse_quote, ItemFn, ReturnType, Stmt};

pub(crate) fn cu_stats_inner(mut input_fn: ItemFn) -> TokenStream {
    let name = input_fn.sig.ident.to_string();

    let log_before = quote! {
            solana_program::msg!("{}: CU before ", #name);
            solana_program::log::sol_log_compute_units();

    };

    let log_after = quote! {
            solana_program::log::sol_log_compute_units();
            solana_program::msg!("{}: CU after", #name);
    };

    let log_before_stmt: Stmt = parse_quote!({#log_before});
    let log_after_stmt: Stmt = parse_quote!({#log_after});

    input_fn.block.stmts.insert(0, log_before_stmt);
    let pos = {
        let len = input_fn.block.stmts.len();
        if let ReturnType::Default = input_fn.sig.output {
            len
        } else {
            len - 2
        }
    };
    input_fn.block.stmts.insert(pos, log_after_stmt);
    quote! {#input_fn}
}
