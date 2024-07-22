use std::collections::HashMap;
use std::sync::{Arc};
use axum::http::{StatusCode};
use axum::{Extension, Json};
use axum::extract::Path;
use axum::response::{IntoResponse};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    name: String,
    password: String
}

#[derive(Clone)]
pub struct Users {
    list: Arc<Mutex<HashMap<u16, User>>>,
}

impl Users {
    pub fn new() -> Self {
        Users {
            list: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn get_users(&self) -> impl IntoResponse {
        let data: HashMap<u16, User> = self.list.lock().await.clone();
        let values: Vec<User> = data.into_values().collect();

        println!("Getting all users");

        (StatusCode::OK, Json(values))
    }

    async fn create_user(&self, Json(new_user): Json<User>) -> impl IntoResponse {
        let mut list = self.list.lock().await;
        println!("Add new user {}", new_user.name);
        let id = (list.len() + 1) as u16;
        list.insert(id, new_user.clone());

        (StatusCode::CREATED, Json(id))
    }

    async fn get_user(&self, id: u16) -> impl IntoResponse{
        let data = self.list.lock().await;
        let user = data.get(&id).cloned();

        match user {
            Some(user) => {
                println!("Found user {}", user.name);
                (StatusCode::CREATED, Json(user)).into_response()
            },
            None => {
                println!("Did not found user with id {id}");
                StatusCode::NOT_FOUND.into_response()
            }
        }
    }

    async fn update_user(&self, id: u16, updated_user: User) -> impl IntoResponse{
        let mut data = self.list.lock().await;
        if data.contains_key(&id) {
            data.insert(id.clone(), updated_user.clone());
            println!("Updated user {id}");
            (StatusCode::OK, Json(updated_user)).into_response()
        } else {
            println!("Did not found user with id {id}");
            StatusCode::NOT_FOUND.into_response()
        }
    }


    async fn delete_user(&self, id: u16) -> impl IntoResponse {
        let mut data = self.list.lock().await;
        if data.contains_key(&id) {
            data.remove(&id);
            println!("Deleted user with id {id}");
            StatusCode::OK
        } else {
            println!("Did not found user with id {id}");
            StatusCode::NOT_FOUND
        }
    }
}

pub async fn get_users_handler(users: Extension<Arc<Users>>) -> impl IntoResponse {
    users.get_users().await
}

pub async fn create_user_handler(users: Extension<Arc<Users>>, Json(new_user): Json<User>) -> impl IntoResponse {
    users.create_user(Json(new_user)).await
}

pub async fn get_user_handler(users: Extension<Arc<Users>>, Path(id): Path<u16>) -> impl IntoResponse {
    return users.get_user(id).await
}

pub async fn update_user_handler(users: Extension<Arc<Users>>, Path(id): Path<u16>, Json(user): Json<User>) -> impl IntoResponse {
    users.update_user(id, user).await
}

pub async fn delete_user_handler(users: Extension<Arc<Users>>, Path(id): Path<u16>) -> impl IntoResponse {
    users.delete_user(id).await
}



