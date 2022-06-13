use crate::document_wrapper::RenderType;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};

use super::{SupportField, SupportType, SupportTypeName};

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
            ty = quote! { Vec<#ty> };
        } else if &name_str == self_ty {
            ty = quote! { Box<#ty> };
        }
        if !f.non_null() {
            ty = quote! { Option<#ty> };
        }
        ty
    }

    fn rename_token<T>(f: &T) -> TokenStream
    where
        T: SupportType,
    {
        let name = f.gql_name();
        let name = Ident::new(name.as_str(), Span::call_site());
        format!(r#"#[graphql(name = "{}")]"#, name).parse().unwrap()
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
            let field_name = Ident::new(&f.field_name(), Span::call_site());
            res = quote!(
                #res
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
        let n = &Self::field_name_token(f);
        let ty = &Self::struct_name_token(f);
        let arguments = Self::arguments_token(f);
        let arguments_variebles = Self::arguments_variebles(f);
        let field = match f.description() {
            // TODO: unwrap(
            // TODO: Some(desc) => format!(r#"/// {}"#, desc).parse().unwrap(),
            Some(desc) => quote! {},
            None => quote!(),
        };
        let gql = &Self::rename_token(f);
        quote!(
            #field
            #gql
            pub async fn #n(&self, ctx: &Context<'_>, #arguments) -> #ty {
                todo!()
            }
        )
    }

    pub fn scalar_fields_token<T>(f: &T) -> TokenStream
    where
        T: SupportTypeName + SupportType,
    {
        let n = &Self::field_name_token(f);
        let ty = &Self::struct_name_token(f);
        let gql = &Self::rename_token(f);
        quote!(
            #gql
            pub async fn #n(&self, ctx: &Context<'_>) -> #ty {
                todo!()
            }
        )
    }

    pub fn field_property_token<T>(f: &T) -> TokenStream
    where
        T: SupportTypeName + SupportType,
    {
        let n = &Self::field_name_token(f);
        let ty = &Self::struct_name_token(f);
        let gql = &Self::rename_token(f);
        quote!(
            #gql
            pub #n : #ty
        )
    }

    pub fn field_property_token_with_recursion<T>(self_ty: &str, f: &T) -> TokenStream
    where
        T: SupportTypeName + SupportType,
    {
        let n = &Self::field_name_token(f);
        let ty = &Self::struct_name_token_with_recursion(self_ty, f);
        let gql = &Self::rename_token(f);
        quote!(
            #gql
            pub #n : #ty
        )
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
