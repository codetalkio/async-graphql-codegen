use crate::model::*;
use async_graphql::{
    Context, Enum, InputObject, InputValueResult, Interface, Object, Scalar, ScalarType, Union,
    Value, ID,
};
#[derive(Debug)]
pub struct Output;
#[Object(name = "Output")]
impl Output {
    #[graphql(name = "linkId")]
    pub async fn link_id(&self, ctx: &Context<'_>) -> Option<i32> {
        todo!()
    }
    #[graphql(name = "role")]
    pub async fn role(&self, ctx: &Context<'_>) -> Option<String> {
        todo!()
    }
    #[graphql(name = "token")]
    pub async fn token(&self, ctx: &Context<'_>) -> Option<String> {
        todo!()
    }
}
