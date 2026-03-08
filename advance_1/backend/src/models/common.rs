use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Debug, Serialize, ToSchema)]
pub struct Response<T> {
    pub message: String,
    pub value: Option<T>,
}
