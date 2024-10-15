use axum::{routing::{get, delete, patch, post}, Router};
use std::sync::Arc;

use crate::{handler::{create_player, create_player_sport, create_session, delete_player, delete_player_sport, delete_session, edit_player, edit_session, get_players, get_session, get_sessions, get_sports_all, health_checker, session_rsvp, update_player_rating}, AppState
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health_checker", get(health_checker))
        .nest("/player", player_router())
        .nest("/sport", sport_router())
        .nest("/player_game", game_player_router())
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

fn sport_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/all", get(get_sports_all))
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

fn game_player_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/rsvp", post(session_rsvp))
}


fn game_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/all", get(get_sessions))
        .route("/:id", get(get_session))
        .route("/create", post(create_session))
        .route("/edit", patch(edit_session))
        .route("/delete", delete(delete_session))
}

