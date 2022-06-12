use crate::model::*;
use async_graphql::{
    Context, Enum, InputObject, InputValueResult, Interface, Object, Scalar, ScalarType, Union,
    Value, ID,
};
#[derive(Debug)]
pub struct Query;
#[Object(name = "Query")]
impl Query {
    #[graphql(name = "work")]
    pub async fn work(&self, ctx: &Context<'_>, input: Option<Input>) -> Option<Output> {
        todo!()
    }
}
