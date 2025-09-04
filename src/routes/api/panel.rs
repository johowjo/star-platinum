use axum::{Json, http::StatusCode};
use sea_orm_pro::{ConfigParser, JsonCfg};

pub async fn panel_config() -> Result<Json<JsonCfg>, (StatusCode, &'static str)> {
    let config = ConfigParser::new()
        .load_config("")
        .unwrap();
    Ok(Json(config))
}
