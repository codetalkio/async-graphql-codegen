use async_graphql_parser::types::{ObjectType, TypeDefinition};

use super::{
    Context, Dependency, FileRender, MutationTypeWrapper, RenderType, SupportField, SupportTypeName,
};

#[derive(Debug)]
pub struct MutationsTypeWrapper<'a, 'b> {
    pub doc: &'a TypeDefinition,
    pub object: &'a ObjectType,
    pub context: &'a Context<'b>,
}

impl<'a, 'b> FileRender for MutationsTypeWrapper<'a, 'b> {
    fn super_module_name(&self) -> String {
        "mutations_type".to_string()
    }
}

impl<'a, 'b> RenderType for MutationsTypeWrapper<'a, 'b> {
    #[must_use]
    fn name(&self) -> String {
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

impl<'a, 'b> MutationsTypeWrapper<'a, 'b> {
    #[must_use]
    pub fn mutations(&self) -> Vec<MutationTypeWrapper> {
        self.object
            .fields
            .iter()
            .map(|f| MutationTypeWrapper {
                doc: &f.node,
                context: self.context,
            })
            .collect()
    }

    pub fn dependencies(&self) -> Vec<Dependency> {
        let mut deps: Vec<Dependency> = self
            .mutations()
            .iter()
            .flat_map(|f| f.dependencies())
            .collect();

        let arg_deps: Vec<Dependency> = self
            .mutations()
            .iter()
            .flat_map(|f| f.arguments_dependencies())
            .collect();
        deps.extend(arg_deps);
        deps.dedup_by(|a, b| a.name == b.name);
        deps
    }
}
