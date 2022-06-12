use itertools::Itertools;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::HashSet;

use super::Dependency;

pub trait Render {
    fn render_dependencies(this: &str, dependencies: Vec<Dependency>) -> TokenStream {
        let mut res = quote!();
        let mut set = HashSet::new();
        set.extend(dependencies.iter().map(|d| &d.name));

        for f in set {
            if f.as_str() == this {
                continue;
            }
            let name = Ident::new(f, Span::call_site());
            res = quote!(
                #res
                use crate::model::#name;
            )
        }
        res
    }
}
