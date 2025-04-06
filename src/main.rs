use axum::{
    extract::State,
    http::{Method, StatusCode},
    response::{IntoResponse, Json},
    routing::{delete, get, post},
    Router,
};
use log::{error, info};
use serde::Serialize;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

mod color_mixer;
mod error;

use color_mixer::{AddColorRequest, ColorMixer, MixingHistory};
use error::ColorMixerError;

// Make ColorMixerError compatible with axum's error handling
impl IntoResponse for ColorMixerError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            ColorMixerError::InvalidColorFormat(_) | ColorMixerError::UnsupportedColor(_) => {
                (StatusCode::BAD_REQUEST, self.to_string())
            }
            ColorMixerError::MaxColorsReached => (StatusCode::BAD_REQUEST, self.to_string()),
            ColorMixerError::NoColors => (StatusCode::BAD_REQUEST, "No colors in mixer".to_string()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        (
            status,
            Json(ErrorResponse {
                error: error_message,
            }),
        )
            .into_response()
    }
}

// Standard error response format
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

// Standard success response format
#[derive(Serialize)]
struct SuccessResponse<T> {
    success: bool,
    data: T,
}

// Response for the current color
#[derive(Serialize)]
struct ColorResponse {
    color: String,
    rgb: (u8, u8, u8),
}

// Handler for adding a color
async fn add_color(
    State(data): State<Arc<RwLock<ColorMixer>>>,
    Json(req): Json<AddColorRequest>,
) -> Result<Json<SuccessResponse<ColorResponse>>, ColorMixerError> {
    // Validate the request
    if req.color.is_empty() {
        error!("Validation error: color cannot be empty");
        return Err(ColorMixerError::InvalidColorFormat("Color cannot be empty".to_string()));
    }

    // Access the color mixer with write lock since we're modifying it
    let mut mixer = data.write().map_err(|e| {
        error!("Failed to acquire write lock: {}", e);
        ColorMixerError::InternalError(format!("Lock error: {}", e))
    })?;

    // Add the color
    mixer.add_color_str(&req.color)?;
    
    // Get the mixed color and save to history
    let color = mixer.get_mixed_color()?;
    mixer.save_to_history()?;
    
    // Return the mixed color
    Ok(Json(SuccessResponse {
        success: true,
        data: ColorResponse {
            color: color.to_hex(),
            rgb: color.rgb(),
        },
    }))
}

// Handler to get the current color
async fn get_current_color(
    State(data): State<Arc<RwLock<ColorMixer>>>,
) -> Result<Json<SuccessResponse<ColorResponse>>, ColorMixerError> {
    // Access the color mixer with write lock since get_mixed_color requires mutable access
    let mut mixer = data.write().map_err(|e| {
        error!("Failed to acquire write lock: {}", e);
        ColorMixerError::InternalError(format!("Lock error: {}", e))
    })?;

    // Get the mixed color
    let color = mixer.get_mixed_color()?;
    
    // Return the mixed color
    Ok(Json(SuccessResponse {
        success: true,
        data: ColorResponse {
            color: color.to_hex(),
            rgb: color.rgb(),
        },
    }))
}

// Handler to clear all colors
async fn clear_colors(
    State(data): State<Arc<RwLock<ColorMixer>>>,
) -> Result<Json<SuccessResponse<&'static str>>, ColorMixerError> {
    // Access the color mixer with write lock since we're modifying it
    let mut mixer = data.write().map_err(|e| {
        error!("Failed to acquire write lock: {}", e);
        ColorMixerError::InternalError(format!("Lock error: {}", e))
    })?;

    // Clear the colors
    mixer.clear();
    
    // Return success
    Ok(Json(SuccessResponse {
        success: true,
        data: "Colors cleared",
    }))
}

// Handler to get the mixing history
async fn get_history(
    State(data): State<Arc<RwLock<ColorMixer>>>,
) -> Result<Json<SuccessResponse<Vec<MixingHistory>>>, ColorMixerError> {
    // Access the color mixer with read lock since we're only reading
    let mixer = data.read().map_err(|e| {
        error!("Failed to acquire read lock: {}", e);
        ColorMixerError::InternalError(format!("Lock error: {}", e))
    })?;

    // Get the history
    let history = mixer.get_history().to_vec();
    
    // Return the history
    Ok(Json(SuccessResponse {
        success: true,
        data: history,
    }))
}

// Status handler
async fn status() -> Json<SuccessResponse<&'static str>> {
    Json(SuccessResponse {
        success: true,
        data: "Color Mixer API is running",
    })
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init_from_env(
        env_logger::Env::default().default_filter_or("info"),
    );
    
    info!("Starting Color Mixer API");

    // Create the shared state
    let color_mixer = Arc::new(RwLock::new(ColorMixer::new()));

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers(Any)
        .max_age(Duration::from_secs(3600));

    // Define API routes
    let api_routes = Router::new()
        .route("/status", get(status))
        .route("/add-color", post(add_color))
        .route("/current-color", get(get_current_color))
        .route("/clear", delete(clear_colors))
        .route("/history", get(get_history));
        
    // Build the complete router
    let app = Router::new()
        .nest("/api", api_routes)
        .with_state(color_mixer)
        .layer(cors)
        .fallback_service(ServeDir::new("static"));
    
    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    info!("Server listening on http://127.0.0.1:8080");
    axum::serve(listener, app).await
}
    
