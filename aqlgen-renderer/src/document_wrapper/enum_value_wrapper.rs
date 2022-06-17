use super::Context;
use async_graphql_parser::types::{
    EnumValueDefinition,
};

use super::{RenderType, UseContext};

#[derive(Debug, Clone)]
pub struct EnumValueWrapper<'a, 'b> {
    pub doc: &'a EnumValueDefinition,
    pub context: &'a Context<'b>,
}

impl<'a, 'b> UseContext for EnumValueWrapper<'a, 'b> {
    fn context(&self) -> &Context {
        self.context
    }
}

impl<'a, 'b> RenderType for EnumValueWrapper<'a, 'b> {
    #[must_use]
    fn gql_name(&self) -> String {
        self.doc.value.node.to_string()
    }

    #[must_use]
    fn description(&self) -> Option<&String> {
        match &self.doc.description {
            Some(f) => Some(&f.node),
            _ => None,
        }
    }
}
