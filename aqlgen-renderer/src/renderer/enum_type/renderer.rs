use async_graphql::value;
use quote::{quote, ToTokens};

use crate::document_wrapper::{EnumTypeWrapper, RenderType};
use proc_macro2::{Ident, Span, TokenStream};

use super::{FileRender, RenderDependencies, Save};

pub struct Renderer<'a, 'b> {
    wrapper_object: &'a EnumTypeWrapper<'a, 'b>,
}

impl<'a, 'b> RenderDependencies for Renderer<'a, 'b> {}

impl<'a, 'b> Save for Renderer<'a, 'b> {
    fn file_name(&self) -> String {
        self.wrapper_object.file_name()
    }

    fn super_module_name(&self) -> Option<String> {
        Some(self.wrapper_object.path().super_module_name)
    }

    fn str_src(&self) -> String {
        Renderer::token_stream(self).to_string()
    }
}

impl<'a, 'b> Renderer<'a, 'b> {
    pub fn create_file(wrapper_object: &'a EnumTypeWrapper<'a, 'b>) {
        let obj = Self { wrapper_object };
        obj.save(wrapper_object.context);
    }

    pub fn new_and_token_stream(wrapper_object: &'a EnumTypeWrapper<'a, 'b>) -> TokenStream {
        let obj = Self { wrapper_object };
        obj.token_stream()
    }

    fn name_tokens(&self) -> TokenStream {
        let name = Ident::new(&self.wrapper_object.name(), Span::call_site());
        quote! { #name }
    }

    fn enum_values_tokens(&self) -> TokenStream {
        let mut values = quote! {};
        for value in self.wrapper_object.enum_values() {
            let value_name = Ident::new(&value.name(), Span::call_site());
            let gql_name = value.gql_name().to_token_stream();
            values = quote!(
                #values
                #[graphql(name = #gql_name)]
                #value_name,
            );
        }
        values
    }

    fn token_stream(&self) -> TokenStream {
        let name = self.name_tokens();
        let enum_values = self.enum_values_tokens();
        let gql_name = self.wrapper_object.gql_name().to_token_stream();
        quote!(
            use async_graphql::*;

            #[derive(Enum, Debug, Copy, Clone, Eq, PartialEq)]
            #[graphql(name = #gql_name)]
            pub enum #name {
                #enum_values
            }
        )
    }

    fn dependencies_token(&self) -> TokenStream {
        quote!(
            // TODO: later better scan deps
            use async_graphql::*;
        )
    }
}
