use quote::{quote, ToTokens};

use proc_macro2::{Ident, Span, TokenStream};

use super::{
    FieldRenderer, FileRender, InputObjectTypeWrapper, RenderDependencies, RenderType, Save,
    SupportField,
};

pub struct Renderer<'a, 'b> {
    wrapper_object: &'a InputObjectTypeWrapper<'a, 'b>,
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
    pub fn create_file(wrapper_object: &'a InputObjectTypeWrapper<'a, 'b>) {
        let obj = Self { wrapper_object };
        obj.save(wrapper_object.context);
    }

    pub fn new_and_token_stream(wrapper_object: &'a InputObjectTypeWrapper<'a, 'b>) -> TokenStream {
        let obj = Self { wrapper_object };
        obj.token_stream()
    }

    fn token_stream(&self) -> TokenStream {
        let field_properties_token = self.field_properties_token();
        let gql_name = self.wrapper_object.gql_name();
        let name = self.wrapper_object.name();
        let deps = self.dependencies_token();

        let param = if gql_name != name {
            quote! {
                #[graphql(name = #gql_name)]
            }
        } else {
            quote! {}
        };

        let name = Ident::new(&self.wrapper_object.name(), Span::call_site());
        quote!(
            #deps

            #[derive(InputObject, Debug)]
            #param
            pub struct #name {
                #field_properties_token
            }
        )
    }

    fn dependencies_token(&self) -> TokenStream {
        let dep = Self::render_dependencies(
            &self.wrapper_object.name(),
            self.wrapper_object.arguments_dependencies(),
        );
        quote!(
            // TODO: later better scan deps
            use async_graphql::*;
            #dep
        )
    }

    fn field_properties_token(&self) -> TokenStream {
        let mut res = quote!();
        self.wrapper_object.fields().iter().for_each(|f| {
            let field_property_token =
                FieldRenderer::field_property_token_with_recursion(&self.wrapper_object.name(), f);
            res = quote!(
                #res
                #field_property_token,
            )
        });
        res
    }
}
