use poem_openapi::{ApiResponse, Object};
use poem_openapi::payload::Json;
use serde::{Deserialize, Serialize};

/// User object
#[derive(Object, Debug, Serialize, Deserialize, Clone)]
pub struct User {
    /// The name of the user.
    pub name: String,

    /// The password of the user.
    password: String
}

#[derive(ApiResponse)]
pub enum CreateResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 201)]
    Created(Json<u16>),
}


#[derive(ApiResponse)]
pub enum GetResponse {
    #[oai(status = 200)]
    Created(Json<User>),

    /// Returns when the user is not found.
    #[oai(status = 404)]
    NotFound,
}

#[derive(ApiResponse)]
pub enum DeleteResponse {
    /// Returns when the user is successfully deleted.
    #[oai(status = 200)]
    Ok,

    /// Returns when the user is not found.
    #[oai(status = 404)]
    NotFound,
}
