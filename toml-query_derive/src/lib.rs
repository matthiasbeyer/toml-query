extern crate proc_macro;
extern crate darling;
extern crate syn;
extern crate quote;

#[cfg(test)]
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
extern crate serde;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Lit,
    Meta,
    MetaNameValue,
};

#[proc_macro_derive(Partial, attributes(location))]
pub fn derive_partial(tokens: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(tokens).unwrap();
    let mut location: Option<String> = None;
    let name = &ast.ident;

    // Iterate over the struct's #[...] attributes
    for option in ast.attrs.into_iter() {
        let option = option.parse_meta().unwrap();
        match option {
            // Match '#[ident = lit]' attributes. Match guard makes it '#[prefix = lit]'
            Meta::NameValue(MetaNameValue{ref ident, ref lit, ..}) if ident == "location" => {
                if let Lit::Str(lit) = lit {
                    location = Some(lit.value());
                }
            },
            _ => {},
            // ...
        }
    }

    let location = location.unwrap();

    let gen = quote! {
        impl<'a> ::toml_query::read::Partial<'a> for #name {
            const LOCATION : &'static str = #location;
            type  Output                  = Self;
        }
    };

    gen.into()
}


