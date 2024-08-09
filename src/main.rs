use std::io::Error;
use poem::{listener::TcpListener, Route, Server, Result, EndpointExt};
use poem::middleware::Cors;
use poem_openapi::{OpenApiService};
use crate::server::{Todos, TodosApi};

mod server;
mod basic;

#[tokio::main]
async fn main() -> Result<(), Error> {

    let users = Todos::new();
    let users_service =
        OpenApiService::new(TodosApi, "My Rust App", "1.0").server("http://localhost:3000");

    let ui = users_service.swagger_ui();
    let spec = users_service.spec();


    let route = Route::new()
        .nest("/", users_service)
        .nest("/api", ui)
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        .with(Cors::new())
        .data(users);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(route)
        .await?;
    Ok(())
}

