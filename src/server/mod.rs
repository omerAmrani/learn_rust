use std::sync::{Arc};
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    name: String,
    id: u32
}

#[derive(Clone)]
pub struct Users {
    list: Arc<Mutex<Vec<User>>>,
}

impl Users {
    pub fn new() -> Self {
        Users {
            list: Arc::new(Mutex::new(Vec::new())),
        }
    }
    async fn get_users(&self) -> impl IntoResponse {
        let data = self.list.lock().await;
        println!("Getting all users");
        (StatusCode::OK, Json(data.clone()))
    }

    async fn create_user(&self, Json(new_user): Json<User>) -> impl IntoResponse {
        let mut data = self.list.lock().await;
        println!("Add new user {}", new_user.name);
        data.push(new_user);
        StatusCode::CREATED
    }
}

pub async fn get_users_handler(users: Extension<Arc<Users>>) -> impl IntoResponse {
    users.get_users().await
}

pub async fn create_user_handler(users: Extension<Arc<Users>>, Json(new_user): Json<User>) -> impl IntoResponse {
    users.create_user(Json(new_user)).await
}





