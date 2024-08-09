use poem_openapi::{ApiResponse, Object};
use poem_openapi::payload::Json;
use serde::{Deserialize, Serialize};

/// Todo object
#[derive(Object, Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    /// The name of the todo.
    pub name: String,

    /// The description of the todo
    description: String,

    /// Is the todo completed
    completed: bool
}

#[derive(ApiResponse)]
pub enum CreateResponse {
    /// Returns when the todo is successfully created.
    #[oai(status = 201)]
    Created(Json<u16>),
}


#[derive(ApiResponse)]
pub enum GetResponse {
    #[oai(status = 200)]
    Created(Json<Todo>),

    /// Returns when the todo is not found.
    #[oai(status = 404)]
    NotFound,
}

#[derive(ApiResponse)]
pub enum DeleteResponse {
    /// Returns when the todo is successfully deleted.
    #[oai(status = 200)]
    Ok,

    /// Returns when the todo is not found.
    #[oai(status = 404)]
    NotFound,
}
