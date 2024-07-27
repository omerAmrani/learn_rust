mod common;

use std::collections::HashMap;
use std::sync::{Arc};
use poem_openapi::{payload::Json, OpenApi, Tags};
use tokio::sync::Mutex;
use poem::{web::Data, Result};
use poem::web::Path;
use crate::server::common::{CreateResponse, DeleteResponse, GetResponse, User};


#[derive(Clone)]
pub struct Users {
    pub(crate) list: Arc<Mutex<HashMap<u16, User>>>,
}

impl Users {
    pub fn new() -> Self {
        Users {
            list: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn get_users(&self) -> Result<Json<Vec<User>>> {
        let data= self.list.lock().await;
        let values: Vec<User> = data.values().cloned().collect();
        
        println!("Getting all users");

        Ok(Json(values))
    }

    async fn create_user(&self, Json(new_user): Json<User>) -> Result<CreateResponse> {
        let mut list = self.list.lock().await;
        println!("Add new user {}", new_user.name);
        let id = (list.len() + 1) as u16;
        list.insert(id, new_user.clone());

        // Ok(Json(id))
        Ok(CreateResponse::Created(Json(id)))

    }

    async fn get_user(&self, id: u16) -> Result<GetResponse> {
        let data = self.list.lock().await;
        let user = data.get(&id).cloned();

        match user {
            Some(user) => {
                println!("Found user {}", user.name);
                Ok(GetResponse::Created(Json(user)))
            },
            None => {
                println!("Did not found user with id {id}");
                Ok(GetResponse::NotFound)
            }
        }
    }

    async fn update_user(&self, id: u16, updated_user: User) -> Result<GetResponse>{
        let mut data = self.list.lock().await;
        if data.contains_key(&id) {
            data.insert(id.clone(), updated_user.clone());
            println!("Updated user {id}");

            Ok(GetResponse::Created(Json(updated_user)))
        } else {
            println!("Did not found user with id {id}");
            Ok(GetResponse::NotFound)
        }
    }


    async fn delete_user(&self, id: u16) -> Result<DeleteResponse> {
        let mut data = self.list.lock().await;
        if data.contains_key(&id) {
            data.remove(&id);
            println!("Deleted user with id {id}");
            Ok(DeleteResponse::Ok)
        } else {
            println!("Did not found user with id {id}");
            Ok(DeleteResponse::NotFound)
        }
    }
}

#[derive(Tags)]
enum ApiTags {
    UsersBuild,
    ById
}


pub struct UsersApi;

#[OpenApi(prefix_path = "/users", tag= "ApiTags::UsersBuild")]
impl UsersApi {
    /// Get all users
    #[oai(path = "/", method = "get")]
    async fn get_users_handler(&self, users: Data<&Users>) -> Result<Json<Vec<User>>> {
        users.get_users().await
    }

    #[oai(path = "/", method = "post")]
    async fn create_user_handler(&self, users: Data<&Users>, Json(new_user): Json<User>) -> Result<CreateResponse> {
        users.create_user(Json(new_user)).await
    }

    #[oai(path = "/:id", method = "get", tag="ApiTags::ById")]
    async fn get_user_handler(&self, users: Data<&Users>, id: Path<u16>) -> Result<GetResponse> {
        return users.get_user(id.0).await
    }

    #[oai(path = "/:id", method = "put", tag="ApiTags::ById")]
    pub async fn update_user_handler(&self, users: Data<&Users>, id: Path<u16>, Json(user): Json<User>) -> Result<GetResponse> {
        users.update_user(id.0, user).await
    }

    #[oai(path = "/:id", method = "delete", tag="ApiTags::ById")]
    pub async fn delete_user_handler(&self, users: Data<&Users>, id: Path<u16>) -> Result<DeleteResponse> {
        users.delete_user(id.0).await
    }
}






