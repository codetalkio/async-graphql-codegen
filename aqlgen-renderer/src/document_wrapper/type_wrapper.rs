use super::{Context, FieldWrapper, InputValueWrapper};
use async_graphql_parser::types::{
    BaseType as BaseTypeDefinition, InputValueDefinition, Type, TypeDefinition,
};
use heck::{ToPascalCase, ToSnakeCase};

pub trait RenderType {
    fn gql_name(&self) -> String;
    fn description(&self) -> Option<&String>;

    #[must_use]
    fn name(&self) -> String {
        self.gql_name().to_pascal_case()
    }

    #[must_use]
    fn field_name(&self) -> String {
        let name = self.name().to_snake_case();
        if syn::parse_str::<syn::Ident>(&name).is_err() {
            format!("_{}", name)
        } else {
            name
        }
    }
}

#[derive(Debug, Clone)]
pub struct ObjectPath {
    pub super_module_name: String,
    pub module_name: String,
    pub name: String,
}

pub type Dependency = ObjectPath;

pub trait FileRender: RenderType {
    #[must_use]
    fn file_name(&self) -> String {
        self.name().to_snake_case()
    }

    fn super_module_name(&self) -> String;

    fn path(&self) -> ObjectPath {
        ObjectPath {
            super_module_name: self.super_module_name(),
            module_name: self.file_name(),
            name: self.name(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BaseType<'a, 'b, T> {
    pub doc: &'a TypeDefinition,
    pub kind: &'a T,
    pub context: &'a Context<'b>,
}

pub trait UseContext {
    fn context(&self) -> &Context;
}

pub trait SupportFields {
    #[must_use]
    fn fields(&self) -> Vec<FieldWrapper>;

    #[must_use]
    fn dependencies(&self) -> Vec<Dependency> {
        let mut deps: Vec<_> = self
            .fields()
            .into_iter()
            .flat_map(|f| f.dependencies())
            .collect();

        let arg_deps: Vec<_> = self
            .fields()
            .iter()
            .flat_map(|f| f.arguments_dependencies())
            .collect();
        deps.extend(arg_deps);
        deps
    }

    fn field_partition(&self) -> (Vec<FieldWrapper>, Vec<FieldWrapper>) {
        //self.fields().into_iter().partition(FieldWrapper::is_scalar)
        self.fields().into_iter().partition(|_| false)
    }

    fn custom_fields(&self) -> Vec<FieldWrapper> {
        self.field_partition().1
    }

    fn scalar_fields(&self) -> Vec<FieldWrapper> {
        self.field_partition().0
    }
}

pub trait SupportField: UseContext {
    fn input_value_types(&self) -> Vec<&InputValueDefinition>;

    fn arguments(&self) -> Vec<InputValueWrapper> {
        self.input_value_types()
            .iter()
            .map(|f| InputValueWrapper {
                doc: f,
                context: self.context(),
            })
            .collect()
    }

    fn fields(&self) -> Vec<InputValueWrapper> {
        self.arguments()
    }

    fn arguments_dependencies(&self) -> Vec<Dependency> {
        self.arguments()
            .iter()
            .flat_map(|f| f.dependencies())
            .collect()
    }
}

pub trait SupportType: RenderType {
    #[must_use]
    fn ty(&self) -> &Type;

    #[must_use]
    fn non_null(&self) -> bool {
        !self.ty().nullable
    }

    #[must_use]
    fn non_null_base(&self) -> bool {
        match self.ty().base {
            BaseTypeDefinition::List(ref list) => !list.nullable,
            _ => false,
        }
    }

    #[must_use]
    fn is_list(&self) -> bool {
        match self.ty().base {
            BaseTypeDefinition::List(_) => true,
            _ => false,
        }
    }

    #[must_use]
    fn gql_type_name(&self) -> String {
        Self::nested_type_name(self.ty())
    }

    #[must_use]
    fn type_name(&self) -> String {
        self.gql_type_name().to_pascal_case()
    }

    #[must_use]
    fn nested_type_name(t: &Type) -> String {
        match &t.base {
            BaseTypeDefinition::Named(name) => name.to_string(),
            BaseTypeDefinition::List(t) => Self::nested_type_name(t),
        }
    }

    #[must_use]
    fn code_type_name(&self) -> String {
        let name = self.gql_type_name();
        match name.as_str() {
            "Boolean" => "bool".to_string(),
            "Int" => "i32".to_string(),
            "Float" => "f64".to_string(),
            "ID" => "ID".to_string(),
            _ => name.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ScalarTypeOnScalar {
    DefaultScalar,
    CustomScalar,
}

pub trait SupportTypeName: SupportType + UseContext {
    fn scalar_type(&self) -> Option<ScalarTypeOnScalar> {
        let names = self.context().scalar_names();
        let name = &self.gql_type_name();
        match name.as_str() {
            "String" | "Boolean" | "Int" | "Float" | "ID" => {
                Some(ScalarTypeOnScalar::DefaultScalar)
            }
            _ => {
                if names.iter().any(|f| f == name) {
                    Some(ScalarTypeOnScalar::CustomScalar)
                } else {
                    None
                }
            }
        }
    }

    fn is_default_scalar(&self) -> bool {
        match &self.scalar_type() {
            Some(t) => match t {
                ScalarTypeOnScalar::DefaultScalar => true,
                _ => false,
            },
            _ => false,
        }
    }

    #[must_use]
    fn module_name(&self) -> Option<String> {
        if self.is_default_scalar() {
            return None;
        }

        let name = self.code_type_name();
        Some(name.to_snake_case())
    }

    #[must_use]
    fn is_scalar(&self) -> bool {
        match &self.scalar_type() {
            Some(_t) => true,
            _ => false,
        }
    }

    fn is_input_object_type(&self) -> bool {
        let names = self.context().input_object_type_names();
        let name = &self.gql_type_name();
        names.iter().any(|f| f == name)
    }

    fn is_union(&self) -> bool {
        let names = self.context().union_names();
        let name = &self.gql_type_name();
        names.iter().any(|f| f == name)
    }

    #[must_use]
    fn is_custom_scalar(&self) -> bool {
        match &self.scalar_type() {
            Some(t) => match t {
                ScalarTypeOnScalar::CustomScalar => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn super_module_name(&self) -> Option<String> {
        if self.is_custom_scalar() {
            return Some("scalar_type".to_string());
        } else if self.is_union() {
            return Some("union_type".to_string());
        } else if self.is_input_object_type() {
            return Some("input_object_type".to_string());
        } else if !self.is_scalar() {
            return Some("object_type".to_string());
        };
        None
    }

    #[must_use]
    fn dependencies(&self) -> Vec<Dependency> {
        match self.super_module_name() {
            Some(super_module_name) => {
                let dep = Dependency {
                    super_module_name,
                    module_name: self.module_name().unwrap(),
                    name: self.type_name(),
                };
                return vec![dep];
            }
            None => vec![],
        }
    }

    fn struct_name(&self) -> String {
        let name = self.code_type_name();
        match (self.non_null(), self.is_list()) {
            (true, false) => name,
            (true, true) => format!("Vec<{}>", name),
            (false, false) => format!("Option<{}>", name),
            (false, true) => format!("Option<Vec<{}>>", name),
        }
    }
}
