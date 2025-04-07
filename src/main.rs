mod color_mixer;
mod error;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::services::ServeDir;
use serde::Serialize;

use crate::color_mixer::{AddColorRequest, ColorMixer};
use crate::error::ColorMixerError;

#[derive(Serialize)]
struct ColorResponse {
    color: String,
    rgb: (u8, u8, u8),
}

type AppState = Arc<RwLock<ColorMixer>>;

async fn add_color(
    State(state): State<AppState>,
    Json(payload): Json<AddColorRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if payload.color.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Color cannot be empty".to_string()));
    }

    if payload.quantity == 0 {
        return Err((StatusCode::BAD_REQUEST, "Quantity must be greater than zero".to_string()));
    }

    let mut mixer = state.write().await;
    mixer.add_colors_str(&payload.color, payload.quantity).map_err(|e| match e {
        ColorMixerError::UnsupportedColor(msg) => (StatusCode::BAD_REQUEST, msg),
        ColorMixerError::MaxColorsReached => {
            (StatusCode::BAD_REQUEST, "Maximum number of colors reached".to_string())
        }
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
    })?;

    let color = mixer.get_mixed_color().map_err(|e| match e {
        ColorMixerError::NoColors => (StatusCode::BAD_REQUEST, "No colors to mix".to_string()),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
    })?;

    Ok(Json(ColorResponse {
        color: color.to_hex(),
        rgb: color.rgb(),
    }))
}

async fn get_current_color(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mixer = state.read().await;
    let color = mixer.get_mixed_color().map_err(|e| match e {
        ColorMixerError::NoColors => (StatusCode::BAD_REQUEST, "No colors to mix".to_string()),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
    })?;

    Ok(Json(ColorResponse {
        color: color.to_hex(),
        rgb: color.rgb(),
    }))
}

async fn clear_colors(State(state): State<AppState>) -> impl IntoResponse {
    state.write().await.clear();
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::init();

    // Create shared state
    let state = Arc::new(RwLock::new(ColorMixer::new()));

    // Build our application with routes
    let app = Router::new()
        .route("/api/color", post(add_color))
        .route("/api/color", get(get_current_color))
        .route("/api/clear", post(clear_colors))
        .with_state(state)
        .fallback_service(ServeDir::new("static"));

    // Run it with hyper on localhost:8080
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

