use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs, Block, FnArg, Ident, ItemFn, Meta, NestedMeta, Pat, PatIdent,
    PatType, Path,
};

#[proc_macro_attribute]
pub fn delegate(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let mut input = parse_macro_input!(input as ItemFn);

    let delegatee = parse_delegatee(&args);
    let delegation = parse_delegation(&input);
    let delegation_args = parse_delegation_args(&input);

    let delegated_block = quote!({ self.#delegatee.#delegation(#(#delegation_args),*) }).into();
    let delegated_block = parse_macro_input!(delegated_block as Block);

    input.block = Box::new(delegated_block);

    quote!(#input).into()
}

fn parse_delegation_args(input: &ItemFn) -> Vec<Ident> {
    input
        .clone()
        .sig
        .inputs
        .into_iter()
        .filter_map(|input| match input {
            FnArg::Typed(PatType {
                attrs: _,
                colon_token: _,
                ty: _,
                pat,
            }) => match pat.as_ref() {
                Pat::Ident(PatIdent {
                    attrs: _,
                    by_ref: _,
                    mutability: _,
                    ident,
                    subpat: _,
                }) => Some(ident.clone()),
                _ => None,
            },
            _ => None,
        })
        .collect()
}

fn parse_delegation(input: &ItemFn) -> Ident {
    input.sig.ident.clone()
}

fn parse_delegatee(args: &[NestedMeta]) -> Ident {
    match &args[0] {
        NestedMeta::Meta(Meta::Path(Path {
            leading_colon: _,
            segments,
        })) => segments[0].ident.clone(),
        _ => unimplemented!(),
    }
}
