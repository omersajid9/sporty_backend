use axum::{routing::{get, delete, patch}, Router};
use std::sync::Arc;

use crate::{handler::{create_game, create_player, create_player_sport, delete_game, delete_player, delete_player_sport, edit_game, edit_player, get_players, health_checker, update_player_rating}, AppState
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health_checker", get(health_checker))
        .nest("/player", player_router())
        .nest("/player_sport", player_sport_router())
        .nest("/rating", rating_router())
        .nest("/game", game_router())
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
        .route("/create", get(create_player_sport))
        .route("/delete", delete(delete_player_sport))
}

fn rating_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/update", get(update_player_rating))
}

fn game_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/create", get(create_game))
        .route("/edit", patch(edit_game))
        .route("/delete", delete(delete_game))
}

