use super::{BaseType, FileRender, RenderType};

pub type ScalarType = ();
pub type ScalarTypeWrapper<'a, 'b> = BaseType<'a, 'b, ScalarType>;

impl<'a, 'b> FileRender for ScalarTypeWrapper<'a, 'b> {
    fn super_module_name(&self) -> String {
        "scalar_type".to_string()
    }
}

impl<'a, 'b> RenderType for ScalarTypeWrapper<'a, 'b> {
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
