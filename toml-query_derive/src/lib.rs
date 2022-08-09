//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

#![warn(rust_2018_idioms)]
#![allow(clippy::collapsible_match)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{Lit, Meta, MetaNameValue};

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
            Meta::NameValue(MetaNameValue { ref path, lit, .. }) if path.is_ident("location") => {
                if let Lit::Str(lit) = lit {
                    location = Some(lit.value());
                }
            }
            _ => {} // ...
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
