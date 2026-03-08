use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Debug, Clone, Serialize, ToSchema, Validate)]
pub struct RequestUser {
    #[validate(length(min = 2, max = 50))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Deserialize, Debug, Clone, Serialize, ToSchema)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub email: String,
}
