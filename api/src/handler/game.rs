use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use skillratings::{
    trueskill::{trueskill_two_teams, TrueSkillConfig, TrueSkillRating},
    Outcomes,
};
use uuid::Uuid;

use crate::{
    model::{
        game::{
            Game, GameData, Rating, Score, Team, TeamMember, TeamScore,
            UserDetails,
        }, player::Player, search::Sport, Username, ID
    },
    schema::game::{PostConfirmScore, PostReportScore},
    AppState,
};

pub async fn get_match(
    axum::extract::Path(id): axum::extract::Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let game = sqlx::query_as!(
        Game,
        "SELECT g.*
        FROM game g
        WHERE g.id = $1",
        id
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let sport = sqlx::query_as!(
        Sport,
        "SELECT * FROM sport WHERE id = (SELECT sport_id FROM session WHERE id = $1)",
        game.id
    ).fetch_one(&data.db)
    .await
    .unwrap();

    let users_team_1 = sqlx::query_as!(
        UserDetails,
        "SELECT p.username, p.id, p.profile_picture
        FROM team_member tm
        INNER JOIN player p ON p.id = tm.player_id
        WHERE tm.team_id = $1",
        game.team_id_1
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let users_team_2 = sqlx::query_as!(
        UserDetails,
        "SELECT p.username, p.id, p.profile_picture
        FROM team_member tm
        INNER JOIN player p ON p.id = tm.player_id
        WHERE tm.team_id = $1",
        game.team_id_2
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let reporter = sqlx::query_as!(
        UserDetails,
        "SELECT p.username, p.id, p.profile_picture FROM player p WHERE id = $1",
        game.reporter_id
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let scores = sqlx::query_as!(
        TeamScore,
        "SELECT s1.score as score_1, s2.score as score_2
        FROM game g
        INNER JOIN score s1 ON s1.game_id = g.id AND s1.team_id = g.team_id_1
        INNER JOIN score s2 ON s2.game_id = g.id AND s2.team_id = g.team_id_2 AND s1.round = s2.round
        WHERE g.id = $1",
        id
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let users_1: Vec<Uuid> = users_team_1
        .iter()
        .map(|u| u.id.clone())
        .collect();
    let users_2: Vec<Uuid> = users_team_2
        .iter()
        .map(|u| u.id.clone())
        .collect();
    let mut players = Vec::new();
    players.extend(users_1);
    players.extend(users_2);

    let accepted = sqlx::query_as!(
        ID,
        "SELECT p.id
        FROM player p
        INNER JOIN score_validation sv ON sv.player_id = p.id
        WHERE sv.game_id = $1
        AND sv.status = 'Yes'",
        id
    )
    .fetch_all(&data.db)
    .await
    .unwrap()
    .iter()
    .map(|u| u.id.clone())
    .collect();

    let total = sqlx::query_as!(
        ID,
        "SELECT p.id
        FROM player p
        INNER JOIN score_validation sv ON sv.player_id = p.id
        WHERE sv.game_id = $1",
        id
    )
    .fetch_all(&data.db)
    .await
    .unwrap()
    .iter()
    .map(|u| u.id.clone())
    .collect();

    let sport = sqlx::query_as!(
        Sport,
        "SELECT * 
        FROM sport
        WHERE id = (SELECT sport_id FROM session WHERE id = $1)",
        id
    ).fetch_one(&data.db)
    .await
    .unwrap();


    let game_data = GameData {
        id: id,
        team_1_users: users_team_1,
        team_2_users: users_team_2,
        reporter_user: reporter,
        sport: sport,
        scores: scores,
        total: total,
        players: players,
        accepted: accepted,
        status: game.status,
        created_at: game.created_at,
    };

    Ok((
        StatusCode::OK,
        Json(json!({"status": "success", "data": json!({"game": game_data})})),
    ))
}

pub async fn report_score(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<PostReportScore>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // let reporter = sqlx::query_as!(
    //     Player,
    //     "SELECT * FROM player WHERE username = $1",
    //     body.reporter_user_id.to_string()
    // )
    // .fetch_one(&data.db)
    // .await
    // .unwrap();

    for report in body.reports {
        let team_1 = sqlx::query_as!(
            Team,
            "INSERT INTO
            team DEFAULT VALUES
            RETURNING *",
        )
        .fetch_one(&data.db)
        .await
        .unwrap();

        let team_2 = sqlx::query_as!(
            Team,
            "INSERT INTO
            team DEFAULT VALUES
            RETURNING *",
        )
        .fetch_one(&data.db)
        .await
        .unwrap();

        for user_id_t1 in report.team_1_user_ids {
            let _ = sqlx::query_as!(
                TeamMember,
                "INSERT INTO
                team_member (team_id, player_id)
                VALUES ($1, $2)",
                team_1.id,
                user_id_t1
            )
            .execute(&data.db)
            .await
            .unwrap();
        }

        for user_id_t2 in report.team_2_user_ids.clone() {
            let _ = sqlx::query_as!(
                TeamMember,
                "INSERT INTO
                team_member (team_id, player_id)
                VALUES ($1, $2)",
                team_2.id,
                user_id_t2
            )
            .execute(&data.db)
            .await
            .unwrap();
        }

        let game = sqlx::query_as!(
            Game,
            "INSERT INTO
            game (session_id, team_id_1, team_id_2, reporter_id)
            VALUES ($1, $2, $3, $4)
            RETURNING *",
            report.session_id,
            team_1.id,
            team_2.id,
            body.reporter_user_id
        )
        .fetch_one(&data.db)
        .await
        .unwrap();

        for (index, score) in report.scores.iter().enumerate() {
            let _ = sqlx::query_as!(
                Score,
                "INSERT INTO
                score (game_id, team_id, score, round)
                VALUES ($1, $2, $3, $4)
                RETURNING *",
                game.id,
                team_1.id,
                score[0],
                index as i32
            )
            .fetch_one(&data.db)
            .await
            .unwrap();

            let _ = sqlx::query_as!(
                Score,
                "INSERT INTO
                score (game_id, team_id, score, round)
                VALUES ($1, $2, $3, $4)
                RETURNING *",
                game.id,
                team_2.id,
                score[1],
                index as i32
            )
            .fetch_one(&data.db)
            .await
            .unwrap();
        }
    }
    Ok((
        StatusCode::OK,
        Json(json!({"status": "success","message": "score reported successfully"})),
    ))
}

pub async fn confirm_score(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<PostConfirmScore>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let game = sqlx::query_as!(
        Game,
        "SELECT *
        FROM game WHERE id = $1",
        body.game_id
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let players_no_confirmation = sqlx::query_as!(
        Player,
        "SELECT p.*
        FROM team_member tm
        INNER JOIN player p ON tm.player_id = p.id
        LEFT JOIN score_validation sv ON sv.game_id = $1 AND sv.player_id = p.id
        WHERE tm.team_id IN ($2, $3) AND sv.status IS NULL",
        game.id,
        game.team_id_1,
        game.team_id_2
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let contains_player = players_no_confirmation.iter().any(|x| x.id == body.user_id.clone());

    let accepted: Vec<String> = sqlx::query_as!(
        Username,
        "SELECT p.username
        FROM player p
        INNER JOIN score_validation sv ON sv.player_id = p.id
        WHERE sv.game_id = $1
        AND sv.status = 'Yes'",
        game.id
    )
    .fetch_all(&data.db)
    .await
    .unwrap()
    .iter()
    .map(|u| u.username.clone())
    .collect();

    let total: Vec<String> = sqlx::query_as!(
        Username,
        "SELECT p.username
        FROM player p
        INNER JOIN score_validation sv ON sv.player_id = p.id
        WHERE sv.game_id = $1",
        game.id
    )
    .fetch_all(&data.db)
    .await
    .unwrap()
    .iter()
    .map(|u| u.username.clone())
    .collect();

    let team_players: Vec<String> = sqlx::query_as!(
        Username,
        "SELECT p.username
        FROM team_member tm
        INNER JOIN player p ON p.id = tm.player_id
        WHERE tm.team_id = $1 OR tm.team_id = $2",
        game.team_id_1,
        game.team_id_2
    )
    .fetch_all(&data.db)
    .await
    .unwrap()
    .iter()
    .map(|u| u.username.clone())
    .collect();

    let accepted_count = accepted.len() + 1;
    let total_count = total.len() + 1;
    let players_count = team_players.len();
    // let total = confirmations.total.unwrap_or(0) + 1;
    // let accepted = confirmations.accepted.unwrap_or(0) + 1;

    if !contains_player {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(json!({"status": "error","message": "player not in game"})),
        ));
    }

    let _score_confirmation = sqlx::query_as!(
        ScoreValidation,
        "INSERT INTO 
        score_validation (game_id, player_id, status)
        VALUES ($1, $2, $3)
        ",
        game.id,
        body.user_id,
        body.confirmation.clone()
    )
    .execute(&data.db)
    .await
    .unwrap();

    if body.confirmation == "No" {
        let rejected_count = total_count - accepted_count + 1;
        if rejected_count * 2 >= players_count {
            let _ = sqlx::query_as!(Game, "UPDATE game SET status = 'No' WHERE id = $1", game.id)
                .execute(&data.db)
                .await
                .unwrap();
        }
        return Ok((
            StatusCode::OK,
            Json(json!({"status": "error","message": "confirmation recorded successfully"})),
        ));
    }

    if (accepted_count + 1) * 2 > players_count && game.status == "Pending" {
        let _ = sqlx::query_as!(
            Game,
            "UPDATE game SET status = 'Yes' WHERE id = $1",
            game.id
        )
        .execute(&data.db)
        .await
        .unwrap();

        let mode = match players_count == 2 {
            true => "single",
            false => "team",
        };

        let teammembers_1 = sqlx::query_as!(
            TeamMember,
            "SELECT * 
            FROM team_member 
            WHERE team_id = $1",
            game.team_id_1
        )
        .fetch_all(&data.db)
        .await
        .unwrap();

        let mut ratings_1: Vec<TrueSkillRating> = Vec::new();

        for teammember in &teammembers_1 {
            let rating_history = sqlx::query_as!(
                Rating,
                "SELECT * 
                FROM rating r
                WHERE r.player_id = $1
                AND r.sport_id IN (SELECT ses.sport_id FROM game g INNER JOIN session ses ON g.session_id = ses.id WHERE g.id = $2)
                AND mode = $3",
                teammember.player_id,
                game.id,
                mode
            ).fetch_optional(&data.db)
            .await
            .unwrap();

            let rating = match rating_history {
                Some(rating) => TrueSkillRating {
                    rating: rating.rating,
                    uncertainty: rating.uncertainity,
                },
                None => TrueSkillRating::default(),
            };
            ratings_1.push(rating);
        }

        let teammembers_2 = sqlx::query_as!(
            TeamMember,
            "SELECT * 
            FROM team_member 
            WHERE team_id = $1",
            game.team_id_2
        )
        .fetch_all(&data.db)
        .await
        .unwrap();

        let mut ratings_2: Vec<TrueSkillRating> = Vec::new();

        for teammember in &teammembers_2 {
            let rating_history = sqlx::query_as!(
                Rating,
                "SELECT * 
                FROM rating r
                WHERE r.player_id = $1
                AND r.sport_id IN (SELECT ses.sport_id FROM game g INNER JOIN session ses ON g.session_id = ses.id WHERE g.id = $2)
                AND mode = $3",
                teammember.player_id,
                game.id,
                mode
            ).fetch_optional(&data.db)
            .await
            .unwrap();

            let rating = match rating_history {
                Some(rating) => TrueSkillRating {
                    rating: rating.rating,
                    uncertainty: rating.uncertainity,
                },
                None => TrueSkillRating::default(),
            };
            ratings_2.push(rating);
        }

        let scores = sqlx::query_as!(
            TeamScore,
            "SELECT s1.score as score_1, s2.score as score_2
            FROM game g
            INNER JOIN score s1 ON s1.game_id = g.id AND s1.team_id = $1
            INNER JOIN score s2 ON s2.game_id = g.id AND s2.team_id = $2 AND s1.round = s2.round
            WHERE g.id = $3",
            game.team_id_1,
            game.team_id_2,
            game.id
        )
        .fetch_all(&data.db)
        .await
        .unwrap();

        let config = TrueSkillConfig::new();
        for score in scores {
            let outcome = if score.score_1 > score.score_2 {
                Outcomes::WIN
            } else if score.score_1 < score.score_2 {
                Outcomes::LOSS
            } else {
                Outcomes::DRAW
            };

            (ratings_1, ratings_2) = trueskill_two_teams(&ratings_1, &ratings_2, &outcome, &config);
        }

        for (index, rating) in ratings_1.iter().enumerate() {
            let _ = sqlx::query_as!(
                Rating,
                "INSERT INTO rating (player_id, sport_id, mode, rating, uncertainity)
                VALUES ($1, (SELECT ses.sport_id FROM game g
                            INNER JOIN session ses ON g.session_id = ses.id
                            WHERE g.id = $2),
                        $3, $4, $5)
                ON CONFLICT (player_id, sport_id, mode) DO UPDATE
                SET rating = $4,
                    uncertainity = $5
                RETURNING *",
                teammembers_1[index].player_id,
                game.id,
                mode,
                rating.rating,
                rating.uncertainty
            )
            .fetch_one(&data.db)
            .await
            .unwrap();
        }

        for (index, rating) in ratings_2.iter().enumerate() {
            let _ = sqlx::query_as!(
                Rating,
                "INSERT INTO rating (player_id, sport_id, mode, rating, uncertainity)
                VALUES ($1, (SELECT ses.sport_id FROM game g
                            INNER JOIN session ses ON g.session_id = ses.id
                            WHERE g.id = $2),
                        $3, $4, $5)
                ON CONFLICT (player_id, sport_id, mode) DO UPDATE
                SET rating = $4,
                    uncertainity = $5
                RETURNING *",
                teammembers_2[index].player_id,
                game.id,
                mode,
                rating.rating,
                rating.uncertainty
            )
            .fetch_one(&data.db)
            .await
            .unwrap();
        }
    }
    Ok((
        StatusCode::OK,
        Json(json!({"status": "success","message": "score confirmed unsuccefully"})),
    ))
}
