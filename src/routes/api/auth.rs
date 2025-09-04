use axum::{Json, http::StatusCode};

static DEMO_TOKEN: &str = "this_is_a_meaningless_token";
static DEMO_PID: &str = "this_is_a_meaningless_pid";
static DEMO_EMAIL: &str = "this_is_a_meaningless_email";

pub async fn user_login() -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
    Ok(Json(serde_json::json!({
        "token": DEMO_TOKEN,
        "pid": DEMO_PID,
        "name": "Demo User",
        "is_verified": true,
    })))
}

pub async fn current_user() -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
    Ok(Json(serde_json::json!({
        "pid": DEMO_PID,
        "name": "Demo User",
        "email": DEMO_EMAIL,
    })))
}
