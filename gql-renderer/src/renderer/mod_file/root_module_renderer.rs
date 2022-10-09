use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use super::{Context, Save};

pub struct Renderer<'a, 'b> {
    pub context: &'a Context<'b>,
}

impl<'a, 'b> Save for Renderer<'a, 'b> {
    fn file_name(&self) -> String {
        "mod".to_string()
    }

    fn super_module_name(&self) -> Option<String> {
        None
    }

    fn str_src(&self) -> String {
        Renderer::token_stream(self).to_string()
    }
}

impl<'a, 'b> Renderer<'a, 'b> {
    pub fn create_file(context: &'a Context<'b>) {
        let obj = Self { context };
        obj.save(context);
    }

    fn token_stream(&self) -> TokenStream {
        let modules = self.modules();
        let uses = self.uses();
        quote!(
            #modules
            #uses
        )
    }

    fn modules(&self) -> TokenStream {
        let mut modules = quote! {};
        self.context.structured_file_paths().iter().for_each(|f| {
            let name = Ident::new(f.0, Span::call_site());
            modules = quote!(
              #modules
              mod #name;
            );
        });
        modules
    }

    fn uses(&self) -> TokenStream {
        let mut uses = quote! {};
        self.context.file_paths().iter().for_each(|f| {
            let super_module_name = Ident::new(&f.super_module_name, Span::call_site());
            let name = Ident::new(&f.name, Span::call_site());
            uses = quote!(
                #uses
                pub use #super_module_name::#name;
            )
        });
        uses
    }
}
