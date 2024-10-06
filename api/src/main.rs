use axum::Router;
use tokio::net::TcpListener;
use std::sync::Arc;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod route;
mod handler;
mod schema;
mod model;

struct AppState {
    pub db: Pool<Postgres>
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let listener = server().await;
    let pool = connect_database().await;
    let state = Arc::new(AppState { db: pool });
    let router = router(state);

    println!("🚀 Server started successfully");
    axum::serve(listener, router)
        .await
        .unwrap();
}

async fn connect_database() -> Pool<Postgres> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    pool
}

async fn server() -> TcpListener {
    let server_address = "0.0.0.0:8080".to_string();
    let listener = TcpListener::bind(server_address)
        .await
        .unwrap();
    listener
}

fn router(appstate: Arc<AppState>) -> Router {
    route::create_router(appstate)
}