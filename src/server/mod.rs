mod common;

use std::collections::HashMap;
use std::sync::{Arc};
use poem_openapi::{payload::Json, OpenApi, Tags};
use tokio::sync::Mutex;
use poem::{web::Data, Result};
use poem::web::Path;
use crate::server::common::{CreateResponse, DeleteResponse, GetResponse, Todo};


#[derive(Clone)]
pub struct Todos {
    pub(crate) list: Arc<Mutex<HashMap<u16, Todo>>>,
}

impl Todos {
    pub fn new() -> Self {
        Todos {
            list: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn get_todos(&self) -> Result<Json<Vec<Todo>>> {
        let data= self.list.lock().await;
        let values: Vec<Todo> = data.values().cloned().collect();
        
        println!("Getting all todos");

        Ok(Json(values))
    }

    async fn create_todo(&self, Json(new_todo): Json<Todo>) -> Result<CreateResponse> {
        let mut list = self.list.lock().await;
        println!("Add new todo {}", new_todo.name);
        let id = (list.len() + 1) as u16;
        list.insert(id, new_todo.clone());

        Ok(CreateResponse::Created(Json(id)))
    }

    async fn todo(&self, id: u16) -> Result<GetResponse> {
        let data = self.list.lock().await;
        let todo = data.get(&id).cloned();

        match todo {
            Some(todo) => {
                println!("Found todo {}", todo.name);
                Ok(GetResponse::Created(Json(todo)))
            }
            None => {
                println!("Did not found todo with id {id}");
                Ok(GetResponse::NotFound)
            }
        }
    }

    async fn update_todo(&self, id: u16, updated_todo: Todo) -> Result<GetResponse>{
        let mut data = self.list.lock().await;
        if data.contains_key(&id) {
            data.insert(id.clone(), updated_todo.clone());
            println!("Updated todo {id}");

            Ok(GetResponse::Created(Json(updated_todo)))
        } else {
            println!("Did not found todo with id {id}");
            Ok(GetResponse::NotFound)
        }
    }


    async fn delete_todo(&self, id: u16) -> Result<DeleteResponse> {
        let mut data = self.list.lock().await;
        if data.contains_key(&id) {
            data.remove(&id);
            println!("Deleted todo with id {id}");
            Ok(DeleteResponse::Ok)
        } else {
            println!("Did not found todo with id {id}");
            Ok(DeleteResponse::NotFound)
        }
    }
}

#[derive(Tags)]
enum ApiTags {
    Todo ,
    ById
}


pub struct TodosApi;

#[OpenApi(prefix_path = "/todos", tag= "ApiTags::Todo")]
impl TodosApi {
    /// Get all todos
    #[oai(path = "/", method = "get")]
    async fn get_todos_handler(&self, todos: Data<&Todos>) -> Result<Json<Vec<Todo>>> {
        todos.get_todos().await
    }

    #[oai(path = "/", method = "post")]
    async fn create_todo_handler(&self, todos: Data<&Todos>, Json(new_todo): Json<Todo>) -> Result<CreateResponse> {
        todos.create_todo(Json(new_todo)).await
    }

    #[oai(path = "/:id", method = "get", tag="ApiTags::ById")]
    async fn get_todo_handler(&self, todos: Data<&Todos>, id: Path<u16>) -> Result<GetResponse> {
        return todos.todo(id.0).await
    }

    #[oai(path = "/:id", method = "put", tag="ApiTags::ById")]
    pub async fn update_todo_handler(&self, todos: Data<&Todos>, id: Path<u16>, Json(todo): Json<Todo>) -> Result<GetResponse> {
        todos.update_todo(id.0, todo).await
    }

    #[oai(path = "/:id", method = "delete", tag="ApiTags::ById")]
    pub async fn delete_todo_handler(&self, todos: Data<&Todos>, id: Path<u16>) -> Result<DeleteResponse> {
        todos.delete_todo(id.0).await
    }
}






