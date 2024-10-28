use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use std::sync::Arc;

use crate::{handler::{game::{confirm_score, get_match, report_score}, health_checker, player::{delete_player, edit_player, get_player, sign_in, sign_up}, search::{explore_sessions, going_sessions, players, reportable_sessions, sports}, session::{create_session, delete_session, edit_session, get_session, rsvp_session, session_players}}, AppState};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health_checker", get(health_checker))
        .nest("/player", player_router())
        .nest("/search", search_router())
        .nest("/game", game_router())
        .nest("/session", session_router())
        .with_state(app_state)
}


fn player_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/profile", get(get_player)) //todo
        .route("/sign-in", post(sign_in))
        .route("/sign-up", post(sign_up))
        .route("/delete", delete(delete_player))
        .route("/edit", patch(edit_player))
}

fn search_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/players", get(players))
        .route("/sports", get(sports))
        .route("/explore_sessions", get(explore_sessions))
        .route("/going_sessions", get(going_sessions))
        .route("/reportable_sessions", get(reportable_sessions))
}

fn session_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/:id", get(get_session))
        .route("/players", get(session_players))
        .route("/create", post(create_session))
        .route("/edit", patch(edit_session))
        .route("/delete", delete(delete_session))
        .route("/rsvp", post(rsvp_session))
}

fn game_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/:id", get(get_match))
        .route("/report", post(report_score))
        .route("/confirm", post(confirm_score))
}

// fn player_router() -> Router<Arc<AppState>> {
//     Router::new()
//         .route("/sign-in", get(sign_in))
//         .route("/sign-up", post(sign_up))
//         .route("/delete", delete(delete_player))
//         .route("/edit", patch(edit_player))
//         .route("/all", get(get_players))
//         .route("/get_profile", get(get_player))
// }

// fn sport_router() -> Router<Arc<AppState>> {
//     Router::new()
//         .route("/all", get(get_sports_all))
// }

// fn session_player_router() -> Router<Arc<AppState>> {
//     Router::new()
//         .route("/rsvp", post(session_rsvp))
// }

// fn score_router() -> Router<Arc<AppState>> {
//     Router::new()
//         .route("/report", post(report_score))
//         .route("/confirm", post(confirm_score))
// }

// fn session_router() -> Router<Arc<AppState>> {
//     Router::new()
//         .route("/all", get(get_sessions))
//         .route("/:id", get(get_session))
//         .route("/create", post(create_session))
//         .route("/edit", patch(edit_session))
//         .route("/delete", delete(delete_session))
//         .route("/upcoming", get(get_upcoming_player_sessions))
//         .route("/to_report", get(get_sessions_played))
//         .route("/get_usernames", get(get_session_rsvp_usernames))
// }

// fn game_router() -> Router<Arc<AppState>> {
//     Router::new()
//         .route("/:id", get(get_match))
// }
