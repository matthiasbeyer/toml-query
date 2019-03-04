extern crate proc_macro;
extern crate darling;
extern crate syn;
#[macro_use] extern crate quote;

#[cfg(test)]
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
extern crate serde;

use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{AttributeArgs, ItemFn};
use syn::parse_macro_input;

#[derive(Debug, FromMeta)]
struct PartialArgs {
    location: String,
}

#[proc_macro_attribute]
pub fn tq_partial_document(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let input     = parse_macro_input!(input as ItemFn);

    let args = match PartialArgs::from_list(&attr_args) {
        Ok(v)  => v,
        Err(e) => return e.write_errors().into(),
    };

    let name     = &input.ident;
    let location = args.location;

    let gen = quote! {
        impl ::toml_query::read::Partial for #name {
            type LOCATION : &'static str = #location;
            type Output = Self;
        }
    };

    gen.into()
}

