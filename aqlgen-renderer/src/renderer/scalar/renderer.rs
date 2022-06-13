use super::{FileRender, RenderType, Save, ScalarTypeWrapper};

use quote::{quote, ToTokens};

use proc_macro2::{Ident, Span, TokenStream};

pub struct Renderer<'a, 'b> {
    wrapper_object: &'a ScalarTypeWrapper<'a, 'b>,
}

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
    pub fn create_file(wrapper_object: &'a ScalarTypeWrapper<'a, 'b>) {
        let obj = Self { wrapper_object };
        obj.save(wrapper_object.context);
    }

    pub fn new_and_token_stream(wrapper_object: &'a ScalarTypeWrapper<'a, 'b>) -> TokenStream {
        let obj = Self { wrapper_object };
        obj.token_stream()
    }

    fn token_stream(&self) -> TokenStream {
        let struct_name = Ident::new(&self.wrapper_object.name(), Span::call_site());
        let name = self.wrapper_object.name();
        let gql_name = self.wrapper_object.gql_name();

        let attribute = if gql_name != name {
            quote! {
                #[Scalar(name = #gql_name)]
            }
        } else {
            quote! {
                #[Scalar]
            }
        };

        quote! {
            use async_graphql::*;

            #[derive(Debug, Clone)]
            pub struct #struct_name(!);

            #attribute
            impl ScalarType for #struct_name {
                fn parse(value: Value) -> InputValueResult<Self> {
                    todo!()
                }

                fn to_value(&self) -> Value {
                    todo!()
                }
            }
        }
    }
}
