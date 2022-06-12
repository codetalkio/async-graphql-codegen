use crate::model::*;
use async_graphql::{
    Context, Enum, InputObject, InputValueResult, Interface, Object, Scalar, ScalarType, Union,
    Value, ID,
};
#[derive(InputObject, Debug)]
pub struct Input {
    #[graphql(name = "linkId")]
    pub link_id: Option<i32>,
    #[graphql(name = "role")]
    pub role: Option<String>,
}
