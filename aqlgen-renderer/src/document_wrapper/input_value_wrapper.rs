use super::Context;
use async_graphql_parser::types::{InputValueDefinition, Type};
use heck::ToSnakeCase;

use super::{RenderType, SupportType, SupportTypeName, UseContext};

#[derive(Debug, Clone)]
pub struct InputValueWrapper<'a, 'b> {
    pub doc: &'a InputValueDefinition,
    pub context: &'a Context<'b>,
}

impl<'a, 'b> SupportType for InputValueWrapper<'a, 'b> {
    fn ty(&self) -> &Type {
        &self.doc.ty.node
    }
}

impl<'a, 'b> RenderType for InputValueWrapper<'a, 'b> {
    #[must_use]
    fn gql_name(&self) -> String {
        self.doc.name.node.to_string()
    }

    #[must_use]
    fn description(&self) -> Option<&String> {
        match &self.doc.description {
            Some(_f) => panic!("Not Implemented"),
            _ => None,
        }
    }
}

impl<'a, 'b> UseContext for InputValueWrapper<'a, 'b> {
    fn context(&self) -> &Context {
        self.context
    }
}

impl<'a, 'b> SupportTypeName for InputValueWrapper<'a, 'b> {}

impl<'a, 'b> InputValueWrapper<'a, 'b> {
    #[must_use]
    pub fn field_name(&self) -> String {
        let name = self.name().to_snake_case();
        if syn::parse_str::<syn::Ident>(&name).is_err() {
            format!("_{}", name)
        } else {
            name
        }
    }
}
