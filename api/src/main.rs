use axum::Router;
use model::{game::Game, session::SessionRsvp};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use std::sync::Arc;
use dotenv::dotenv;
use sqlx::{postgres::{PgListener, PgPoolOptions}, Pool, Postgres};

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
    let router = router(state.clone());

    tokio::spawn(async move {
        listen_for_notifications(state).await;
    });

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
    let cors = CorsLayer::new()
    .allow_origin(Any)  // Allow any origin for development
    .allow_methods(Any) // Allow any HTTP method (GET, POST, etc.)
    .allow_headers(Any); // Allow any headers

    route::create_router(appstate).layer(cors)
}


async fn listen_for_notifications(pool: Arc<AppState>) {
    let pool_clone1 = pool.clone();
    let pool_clone2 = pool.clone();
    let pool_clone3 = pool.clone();

    tokio::spawn(async move {
        listen_for_session_rsvp_channel(pool_clone1, "session_rsvp_channel").await;
    });

    tokio::spawn(async move {
        listen_for_game_channel(pool_clone2, "game_channel").await;
    });

    tokio::spawn(async move {
        listen_for_game_status_change_channel(pool_clone3, "game_status_change_channel").await;
    });
}

// This function takes a channel name as a parameter and listens for notifications on that channel
async fn listen_for_session_rsvp_channel(pool: Arc<AppState>, channel: &str) {
    let mut listener = PgListener::connect_with(&pool.db)
        .await
        .expect("Failed to connect to listen for notifications");

    listener.listen(channel).await.expect("Failed to listen to channel");

    println!("🔔 Listening for session rsvp notifications on {}...", channel);

    while let Ok(notification) = listener.recv().await {
        let parsed: SessionRsvp = serde_json::from_str(notification.payload()).unwrap();
        handler::notifications::session_rsvp_channel(pool.clone(), parsed).await;
    }
}

async fn listen_for_game_channel(pool: Arc<AppState>, channel: &str) {
    let mut listener = PgListener::connect_with(&pool.db)
        .await
        .expect("Failed to connect to listen for notifications");

    listener.listen(channel).await.expect("Failed to listen to channel");

    println!("🔔 Listening for game channel notifications on {}...", channel);

    while let Ok(notification) = listener.recv().await {
        let parsed: Game = serde_json::from_str(notification.payload()).unwrap();
        handler::notifications::game_channel(pool.clone(), parsed).await;
    }
}

async fn listen_for_game_status_change_channel(pool: Arc<AppState>, channel: &str) {
    let mut listener = PgListener::connect_with(&pool.db)
        .await
        .expect("Failed to connect to listen for notifications");

    listener.listen(channel).await.expect("Failed to listen to channel");

    println!("🔔 Listening for game status change channel notifications on {}...", channel);

    while let Ok(notification) = listener.recv().await {
        let parsed: Game = serde_json::from_str(notification.payload()).unwrap();
        handler::notifications::game_status_change_channel(pool.clone(), parsed).await;
    }
}