use std::io::Error;
use poem::{listener::TcpListener, Route, Server, Result, EndpointExt};
use poem::middleware::Cors;
use poem_openapi::{OpenApiService};
use crate::server::{Todos, TodosApi};

mod server;
mod basic;

#[tokio::main]
async fn main() -> Result<(), Error> {

    let todos = Todos::new();
    let todos_service =
        OpenApiService::new(TodosApi, "My Rust App", "1.0").server("http://0.0.0.0:3000");

    let ui = todos_service.swagger_ui();
    let spec = todos_service.spec();

    let route = Route::new()
        .nest("/", todos_service)
        .nest("/api", ui)
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        .with(Cors::new())
        .data(todos);

    println!("Server is Running on port 3000");
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(route)
        .await?;


    Ok(())
}

