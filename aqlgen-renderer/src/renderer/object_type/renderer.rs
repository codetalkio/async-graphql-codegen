use quote::{quote, ToTokens};

use proc_macro2::{Ident, Span, TokenStream};

use super::{
    FieldRenderer, FileRender, ObjectTypeWrapper, RenderDependencies, RenderType, Save,
    SupportFields,
};

use heck::ToPascalCase;

pub struct Renderer<'a, 'b> {
    wrapper_object: &'a ObjectTypeWrapper<'a, 'b>,
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
    pub fn create_file(wrapper_object: &'a ObjectTypeWrapper<'a, 'b>) {
        let obj = Self { wrapper_object };
        obj.save(wrapper_object.context);
    }

    pub fn new_and_token_stream(wrapper_object: &'a ObjectTypeWrapper<'a, 'b>) -> TokenStream {
        let obj = Self { wrapper_object };
        obj.token_stream()
    }

    fn token_stream(&self) -> TokenStream {
        let fields = self.custom_fields_token();
        let struct_properties = self.struct_properties_token();
        let scalar_fields_token = self.scalar_fields_token();
        let dependencies = self.dependencies_token();

        let gql_name = self.wrapper_object.gql_name();
        let name = self.wrapper_object.name();

        let attribute = if gql_name != name {
            quote! {
                #[Object(name = #gql_name)]
            }
        } else {
            quote! {
                #[Object]
            }
        };

        let name = Ident::new(&name, Span::call_site());
        quote! {
            #dependencies

            // TODO: add comment
            #[derive(Debug)]
            pub struct #name;

            #attribute
            impl #name {
                #fields
                #scalar_fields_token
            }
        }
    }

    fn dependencies_token(&self) -> TokenStream {
        let dep = Self::render_dependencies(
            &self.wrapper_object.name(),
            self.wrapper_object.dependencies(),
        );
        quote!(
            // TODO: later better scan deps
            use async_graphql::*;
            #dep
        )
    }

    fn gql_name_token(&self) -> TokenStream {
        self.wrapper_object.gql_name().to_token_stream()
    }

    fn name_token(&self) -> TokenStream {
        let name = Ident::new(&self.wrapper_object.name(), Span::call_site());
        quote!(#name)
    }

    fn struct_properties_token(&self) -> TokenStream {
        let mut properties = quote! {};
        self.wrapper_object.scalar_fields().iter().for_each(|f| {
            let field_property = FieldRenderer::field_property_token(f);
            properties = quote!(
                #properties
                #field_property,
            );
        });
        properties
    }

    fn custom_fields_token(&self) -> TokenStream {
        let mut fields = quote! {};
        self.wrapper_object.custom_fields().iter().for_each(|f| {
            let field = &FieldRenderer::custom_field_token(f);
            fields = quote!(
                #fields
                #field
            );
        });
        fields
    }

    fn scalar_fields_token(&self) -> TokenStream {
        let mut scalar_fields = quote! {};
        self.wrapper_object.scalar_fields().iter().for_each(|f| {
            let field = FieldRenderer::scalar_fields_token(f);
            scalar_fields = quote!(
                #scalar_fields
                #field
            );
        });
        scalar_fields
    }
}
