use crate::server::{create_user_handler, get_user_handler, get_users_handler, update_user_handler, Users};
use axum::{Router, routing::get};
use std::sync::Arc;
use axum::extract::Extension;

mod server;
mod basic;

#[tokio::main]
async fn main() {
    let users = Arc::new(Users::new());

    let user_routes = Router::new()
        .route("/", get(get_users_handler).post(create_user_handler))
        .route("/:id", get(get_user_handler).put(update_user_handler));

    let app = Router::new()
        .nest("/users", user_routes)
        .layer(Extension(users));

  println!("Started running on 0.0.0.0:3000 ");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

