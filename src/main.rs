use crate::server::{create_user_handler, get_user_handler, get_users_handler, Users};
use axum::{Router, routing::get, routing::post};
use std::sync::Arc;
use axum::extract::Extension;

mod server;
mod basic;

#[tokio::main]
async fn main() {
    let users = Arc::new(Users::new());

    let app = Router::new()
        .route("/users", get(get_users_handler))
        .route("/users", post(create_user_handler))
        .route("/users/:id", get(get_user_handler))

        .layer(Extension(users));

  println!("Started running on 0.0.0.0:3000 ");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

