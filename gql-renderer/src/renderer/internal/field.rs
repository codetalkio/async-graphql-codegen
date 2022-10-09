use crate::document_wrapper::RenderType;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use super::{SupportField, SupportType, SupportTypeName};
use heck::ToLowerCamelCase;

pub trait Render {
    fn field_name_token<T>(f: &T) -> TokenStream
    where
        T: SupportType,
    {
        let name = f.field_name();
        let name = Ident::new(name.as_str(), Span::call_site());
        quote!(#name)
    }

    fn struct_name_token<T>(f: &T) -> TokenStream
    where
        T: SupportType,
    {
        let name = f.code_type_name();
        let name = Ident::new(&name, Span::call_site());
        match (f.non_null(), f.is_list()) {
            (true, false) => quote!(#name),
            // TODO: may be slice data
            (true, true) => quote!(Vec<#name>),
            (false, false) => quote!(Option<#name>),
            // TODO: may be slice data
            (false, true) => quote!(Option<Vec<#name>>),
        }
    }

    fn struct_name_token_with_recursion<T>(self_ty: &str, f: &T) -> TokenStream
    where
        T: SupportType,
    {
        let name_str = f.code_type_name();
        let name = Ident::new(&name_str, Span::call_site());

        let mut ty = quote! { #name };
        if f.is_list() {
            if f.non_null_base() {
                ty = quote! { Vec<#ty> };
            } else {
                ty = quote! { Vec<Option<#ty>> };
            }
        } else if name_str == *self_ty {
            ty = quote! { Box<#ty> };
        }
        if !f.non_null() {
            ty = quote! { Option<#ty> };
        }
        ty
    }

    fn rename_token(name: &str, gql_name: &str) -> TokenStream {
        if name.to_lower_camel_case() != gql_name {
            quote! { #[graphql(name = #gql_name)] }
        } else {
            quote! {}
        }
    }
}

pub struct Renderer {}

impl Render for Renderer {}

impl Renderer {
    fn arguments_token<T>(f: &T) -> TokenStream
    where
        T: SupportTypeName + SupportType + SupportField,
    {
        let mut res = quote!();
        f.arguments().iter().for_each(|f| {
            let code_type_name = Self::struct_name_token(f);
            let gql_name = f.gql_name();
            let field_name = f.field_name();
            let param = Self::rename_token(&field_name, &gql_name);
            let field_name = Ident::new(&field_name, Span::call_site());
            res = quote!(
                #res
                #param
                #field_name: #code_type_name,
            );
        });
        res
    }

    fn arguments_variebles<T>(f: &T) -> TokenStream
    where
        T: SupportTypeName + SupportType + SupportField,
    {
        let mut res = quote!();
        f.arguments().iter().for_each(|f| {
            let field_name = Ident::new(&f.field_name(), Span::call_site());
            res = quote!(
                #res
                #field_name,
            );
        });
        res
    }

    pub fn custom_field_token<T>(f: &T) -> TokenStream
    where
        T: SupportTypeName + SupportType + SupportField,
    {
        let ty = &Self::struct_name_token(f);
        let arguments = &Self::arguments_token(f);

        let name = f.field_name();
        let gql_name = f.gql_name();
        let param = &Self::rename_token(&name, &gql_name);
        let name = Ident::new(&name, Span::call_site());
        quote! {
            #param
            pub async fn #name(&self, ctx: &Context<'_>, #arguments) -> #ty {
                todo!()
            }
        }
    }

    pub fn scalar_fields_token<T>(f: &T) -> TokenStream
    where
        T: SupportTypeName + SupportType,
    {
        let ty = &Self::struct_name_token(f);

        let name = f.field_name();
        let gql_name = f.gql_name();
        let param = Self::rename_token(&name, &gql_name);
        let name = Ident::new(&name, Span::call_site());
        quote!(
            #param
            pub async fn #name(&self, ctx: &Context<'_>) -> #ty {
                todo!()
            }
        )
    }

    pub fn field_property_token<T>(f: &T) -> TokenStream
    where
        T: SupportTypeName + SupportType,
    {
        let name = f.field_name();
        let gql_name = f.gql_name();
        let param = Self::rename_token(&name, &gql_name);

        let name = Ident::new(&name, Span::call_site());
        let ty = &Self::struct_name_token(f);
        quote! {
            #param
            pub #name : #ty
        }
    }

    pub fn field_property_token_with_recursion<T>(self_ty: &str, f: &T) -> TokenStream
    where
        T: SupportTypeName + SupportType,
    {
        let name = f.field_name();
        let gql_name = f.gql_name();
        let param = Self::rename_token(&name, &gql_name);

        let name = Ident::new(&name, Span::call_site());
        let ty = &Self::struct_name_token_with_recursion(self_ty, f);
        quote! {
            #param
            pub #name : #ty
        }
    }

    pub fn field_interface_token<T>(f: &T) -> TokenStream
    where
        T: SupportTypeName + SupportType,
    {
        let n = f.field_name();
        let ty = f.struct_name();
        quote!(
            field(name = #n, type = #ty)
        )
    }
}
