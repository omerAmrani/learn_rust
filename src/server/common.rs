use poem_openapi::{ApiResponse, Object};
use poem_openapi::payload::Json;
use serde::{Deserialize, Serialize};

#[derive(Object, Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    password: String
}

#[derive(ApiResponse)]
pub enum CreateResponse {
    #[oai(status = 201)]
    Created(Json<u16>),
}


#[derive(ApiResponse)]
pub enum GetResponse {
    #[oai(status = 200)]
    Created(Json<User>),

    #[oai(status = 404)]
    NotFound,
}

#[derive(ApiResponse)]
pub enum DeleteResponse {
    #[oai(status = 200)]
    Ok,

    #[oai(status = 404)]
    NotFound,
}
