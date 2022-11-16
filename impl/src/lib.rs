use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, AttributeArgs, Block, FnArg, Ident, ItemFn, Meta, NestedMeta, Pat, Result,
    TraitItemMethod, Visibility,
};

enum Delegation {
    Direct,
    Called,
}

#[derive(Debug)]
struct DelegatedMethod {
    vis: syn::Visibility,
    method: syn::TraitItemMethod,
}

impl Parse for DelegatedMethod {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            vis: input.parse::<Visibility>().unwrap_or(Visibility::Inherited),
            method: input.parse::<TraitItemMethod>()?,
        })
    }
}

#[proc_macro_attribute]
pub fn delegate(args: TokenStream, input: TokenStream) -> TokenStream {
    delegate_impl(args, input, Delegation::Direct)
}

#[proc_macro_attribute]
pub fn delegate_call(args: TokenStream, input: TokenStream) -> TokenStream {
    delegate_impl(args, input, Delegation::Called)
}

fn delegate_impl(args: TokenStream, input: TokenStream, mode: Delegation) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as DelegatedMethod);

    let delegatee = parse_delegatee(&args);
    let delegation = match mode {
        Delegation::Direct => parse_delegation(&input),
        Delegation::Called => parse_delegation_call(&args),
    };
    let args = parse_delegation_args(&input);

    let block = quote!({ self.#delegatee.#delegation(#(#args),*) }).into();
    let block = parse_macro_input!(block as Block);

    let output = ItemFn {
        vis: input.vis,
        attrs: input.method.attrs,
        sig: input.method.sig,
        block: Box::new(block),
    };

    quote!(#output).into()
}

fn parse_delegation_args(input: &DelegatedMethod) -> Vec<Ident> {
    input
        .method
        .sig
        .inputs
        .iter()
        .filter_map(|input| match input {
            FnArg::Typed(pat) => match pat.pat.as_ref() {
                Pat::Ident(ident) => Some(ident.ident.clone()),
                _ => None,
            },
            _ => None,
        })
        .collect()
}

fn parse_delegation(input: &DelegatedMethod) -> Ident {
    input.method.sig.ident.clone()
}

fn parse_delegation_call(args: &[NestedMeta]) -> Ident {
    parse_args_ident(args, 1)
}

fn parse_delegatee(args: &[NestedMeta]) -> Ident {
    parse_args_ident(args, 0)
}

fn parse_args_ident(args: &[NestedMeta], idx: usize) -> Ident {
    match &args[idx] {
        NestedMeta::Meta(Meta::Path(path)) => path.segments[0].ident.clone(),
        _ => unreachable!(),
    }
}
