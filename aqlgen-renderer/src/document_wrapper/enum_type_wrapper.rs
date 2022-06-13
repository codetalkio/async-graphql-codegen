use crate::document_wrapper::EnumValueWrapper;
use async_graphql_parser::types::{EnumType, EnumValueDefinition, UnionType};

use super::{BaseType, Dependency, FileRender, ObjectTypeWrapper, RenderType};

pub type EnumTypeWrapper<'a, 'b> = BaseType<'a, 'b, EnumType>;

impl<'a, 'b> FileRender for EnumTypeWrapper<'a, 'b> {
    fn super_module_name(&self) -> String {
        "enum_type".to_string()
    }
}

impl<'a, 'b> RenderType for EnumTypeWrapper<'a, 'b> {
    #[must_use]
    fn gql_name(&self) -> String {
        self.doc.name.node.as_str().into()
    }

    #[must_use]
    fn description(&self) -> Option<&String> {
        match &self.doc.description {
            Some(_f) => panic!("Not Implemented"),
            _ => None,
        }
    }
}

impl<'a, 'b> EnumTypeWrapper<'a, 'b> {
    pub fn enum_values(&self) -> Vec<EnumValueWrapper> {
        self.kind
            .values
            .iter()
            .map(|f| EnumValueWrapper {
                doc: &f.node,
                context: self.context,
            })
            .collect()
    }
}
