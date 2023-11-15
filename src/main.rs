use axum::run::run;

pub mod axum;
pub mod helpers;

#[tokio::main]
async fn main() {
    run().await;
}
