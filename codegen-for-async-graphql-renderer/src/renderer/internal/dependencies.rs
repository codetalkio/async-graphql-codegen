use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use super::Dependency;

pub trait Render {
    fn render_dependencies(dependencies: Vec<Dependency>) -> TokenStream {
        let mut res = quote!();
        dependencies.iter().for_each(|f| {
            let name = Ident::new(&f.name, Span::call_site());
            res = quote!(
                #res
                use crate::model::#name;
            )
        });
        res
    }
}
