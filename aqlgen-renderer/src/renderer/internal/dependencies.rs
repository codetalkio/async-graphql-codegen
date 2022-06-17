
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::HashSet;

use super::Dependency;

pub trait Render {
    // TODO: Later
    fn render_dependencies(this: &str, dependencies: Vec<Dependency>) -> TokenStream {
        let mut res = quote!();
        let mut set = HashSet::new();
        set.extend(dependencies.into_iter().map(|d| d.name));

        let set_size = set.len();
        for name in set {
            if name.as_str() == this {
                continue;
            }
            let name = Ident::new(&name, Span::call_site());
            res = quote!(
                #res
                #name,
            )
        }
        if set_size != 0 {
            quote! {
                use super::super::{
                    #res
                };
            }
        } else {
            quote! {}
        }
    }

    //fn render_dependencies(this: &str, dependencies: Vec<Dependency>) -> TokenStream {
    //    quote! {
    //        use crate::model::*;
    //    }
    //}
}
