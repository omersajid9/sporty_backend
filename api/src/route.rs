use axum::{routing::{get, delete, patch}, Router};
use std::sync::Arc;

use crate::{AppState, 
    handler::{health_checker, create_player, delete_player, edit_player, get_players}
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health_checker", get(health_checker))
        .nest("/player", player_router())
        .with_state(app_state)
}

fn player_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/create", get(create_player))
        .route("/delete", delete(delete_player))
        .route("/edit", patch(edit_player))
        .route("/all", get(get_players))
}

fn player_sport_router() -> Router<Arc<AppState>> {
    Router::new()
        // Add routes for player sports here
        // Example: .route("/<player_id>/sport/<sport_id>", get(get_player_sport))
}