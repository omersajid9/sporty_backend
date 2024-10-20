use axum::{routing::{get, delete, patch, post}, Router};
use std::sync::Arc;

use crate::{handler::{confirm_score, create_player, create_player_sport, create_session, delete_player, delete_player_sport, delete_session, edit_player, edit_session, get_match, get_players, get_session, get_session_rsvp_usernames, get_sessions, get_sessions_played, get_sports_all, get_upcoming_player_sessions, get_user_profile, health_checker, report_score, session_rsvp, sign_in, sign_up, update_player_rating}, AppState
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health_checker", get(health_checker))
        .nest("/player", player_router())
        .nest("/sport", sport_router())
        .nest("/player_session", session_player_router())
        .nest("/player_sport", player_sport_router())
        .nest("/rating", rating_router())
        .nest("/session", session_router())
        .nest("/game", game_router())
        .nest("/score", score_router())
        .with_state(app_state)
}

fn player_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/sign-in", get(sign_in))
        .route("/sign-up", post(sign_up))
        .route("/create", get(create_player))
        .route("/delete", delete(delete_player))
        .route("/edit", patch(edit_player))
        .route("/all", get(get_players))
        .route("/get_profile", get(get_user_profile))
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

fn session_player_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/rsvp", post(session_rsvp))
}

fn score_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/report", post(report_score))
        .route("/confirm", post(confirm_score))
}

fn session_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/all", get(get_sessions))
        .route("/:id", get(get_session))
        .route("/create", post(create_session))
        .route("/edit", patch(edit_session))
        .route("/delete", delete(delete_session))
        .route("/upcoming", get(get_upcoming_player_sessions))
        .route("/to_report", get(get_sessions_played))
        .route("/get_usernames", get(get_session_rsvp_usernames))
}

fn game_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/:id", get(get_match))
}

