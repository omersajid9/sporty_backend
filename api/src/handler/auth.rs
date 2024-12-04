use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::{
    auth::Claims,
    model::player::Player,
    schema::auth::{LogIn, RefreshToken, SendOptSms},
    util::twilio::TwilioService,
    AppState,
};
use rnglib::{Language, RNG};

pub async fn send_opt_sms(
    axum::extract::Json(body): axum::extract::Json<SendOptSms>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let otp_details = TwilioService::send_otp(&body.phone).await;
    match otp_details {
        Ok(_success) => {
            return Ok(StatusCode::OK);
        }
        Err(err) => {
            let data = json!({
                "status": "error",
                "error": err
            });
            return Err((StatusCode::BAD_REQUEST, Json(data)));
        }
    }
}

pub async fn log_in(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<LogIn>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Check if user already exists
    let user_present = sqlx::query_as!(
        Player,
        "SELECT * FROM player WHERE auth_type = $1 AND auth_id = $2",
        body.auth_type.clone(),
        body.auth_id.clone()
    )
    .fetch_optional(&data.db)
    .await
    .unwrap();

    // Validate or create player based on auth_type
    let player = match user_present {
        Some(player) => {
            // Handle existing players based on auth type
            match body.auth_type.to_lowercase().as_str() {
                "apple" => player,
                "email" => {
                    if let (Some(password), Some(try_password)) = (player.password.clone(), body.passcode.clone()) {
                        if password != try_password {
                            return Err((
                                StatusCode::BAD_REQUEST,
                                Json(json!({"message": "Incorrect password."})),
                            ));
                        }
                    } else {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            Json(json!({"message": "Password not provided."})),
                        ));
                    }
                    player
                }
                "phone" => {
                    if let Some(opt) = body.passcode.clone() {
                        let verify = TwilioService::verify_otp(&body.auth_id, &opt).await;
                        if verify.is_err() {
                            return Err((
                                StatusCode::BAD_REQUEST,
                                Json(json!({"message": "Incorrect OTP."})),
                            ));
                        }
                    } else {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            Json(json!({"message": "OTP not provided."})),
                        ));
                    }
                    player
                }
                _ => {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(json!({"message": "Invalid auth type."})),
                    ));
                }
            }
        }
        None => {
            // Generate new username
            let mut rng = RNG::try_from(&Language::Fantasy).unwrap();
            let first_name = rng.generate_short().to_lowercase();
            rng = RNG::try_from(&Language::Elven).unwrap();
            let last_name = rng.generate_short().to_lowercase();
            let username = format!("{}_{}", first_name, last_name);

            // Insert new player into database
            match body.auth_type.to_lowercase().as_str() {
                "apple" | "phone" => {
                    if body.auth_type == "phone" {
                        if let Some(opt) = body.passcode.clone() {
                            let verify = TwilioService::verify_otp(&body.auth_id, &opt).await;
                            if verify.is_err() {
                                return Err((
                                    StatusCode::BAD_REQUEST,
                                    Json(json!({"message": "Incorrect OTP."})),
                                ));
                            }
                        } else {
                            return Err((
                                StatusCode::BAD_REQUEST,
                                Json(json!({"message": "OTP not provided."})),
                            ));
                        }
                        sqlx::query_as!(
                            Player,
                            "INSERT INTO player (username, auth_type, auth_id)
                             VALUES ($1, $2, $3)
                             RETURNING *",
                            username,
                            body.auth_type,
                            body.auth_id
                        )
                        .fetch_one(&data.db)
                        .await
                        .unwrap()
                    } else {
                        sqlx::query_as!(
                            Player,
                            "INSERT INTO player (username, first_name, last_name, auth_type, auth_id)
                             VALUES ($1, $2, $3, $4, $5)
                             RETURNING *",
                            username,
                            body.first_name.unwrap_or("".to_string()),
                            body.last_name.unwrap_or("".to_string()),
                            body.auth_type,
                            body.auth_id
                        )
                        .fetch_one(&data.db)
                        .await
                        .unwrap()
                    }

                }
                "email" => {
                    if let Some(password) = body.passcode.clone() {
                        sqlx::query_as!(
                            Player,
                            "INSERT INTO player (username, password, auth_type, auth_id)
                             VALUES ($1, $2, $3, $4)
                             RETURNING *",
                            username,
                            password,
                            body.auth_type,
                            body.auth_id
                        )
                        .fetch_one(&data.db)
                        .await
                        .unwrap()
                    } else {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            Json(json!({"message": "Password not provided."})),
                        ));
                    }
                }
                _ => {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(json!({"message": "Invalid auth type."})),
                    ));
                }
            }
        }
    };

    let claims = Claims::new(player.id, 1); // Token valid for 24 hours
    let token = claims
        .generate_token()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "Token generation failed."}))))?;

    let player_response = json!({
        "status": "success",
        "message": "Player signed in successfully",
        "auth_token": {"token": token},
        "profile": {"user": player},
    });

    Ok((StatusCode::OK, Json(player_response)))
}


pub async fn refresh_token(
    axum::extract::Json(body): axum::extract::Json<RefreshToken>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let new_claims = Claims::new(body.user_id, 1);
    let new_token = new_claims
        .generate_token()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "Token generation failed."}))))?;

    let response = json!({
        "status": "success",
        "message": "Token refreshed successfully",
        "auth_token": {"token": new_token},
    });
    
    Ok((StatusCode::OK, Json(response)))
}