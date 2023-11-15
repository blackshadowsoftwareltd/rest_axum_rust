use axum::Server;

use crate::{axum::routes::routes, helpers::addr::ip_address};

pub async fn run() {
    println!("Start");

    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = routes().await;

    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();

    let addr = ip_address().await;
    println!("Listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
