use std::sync::Arc;
use axum::{routing::{get, post}, Router, Json};
use crate::server::{User, Users};

mod server;
mod basic;

#[tokio::main]
async fn main() {
  let users = Arc::new(Users::new());

  let app = Router::new()
      .route("/users", get({
        let users = Arc::clone(&users);
        move || {
          let users = Arc::clone(&users);
          async move { users.get_users().await }
        }
      }))
      .route("/users", post({
        let users = Arc::clone(&users);
        move |body: Json<User>| {
          let users = Arc::clone(&users);
          async move { users.create_user(body).await }
        }
      }));

  println!("Started running on 0.0.0.0:3000 ");

  let listener = tokio::net::TcpListener::
    bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();

}