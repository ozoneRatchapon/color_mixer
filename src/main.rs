use actix_cors::Cors;
use actix_files as fs;
use actix_web::{
    middleware, web, App, Error as ActixError, HttpResponse, HttpServer, Responder,
    ResponseError,
};
use serde::Serialize;
use std::sync::Arc;
use std::sync::RwLock;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;
use validator::Validate;

mod color_mixer;
mod error;

use color_mixer::{AddColorRequest, ColorMixer};
use error::ColorMixerError;

// Implement ResponseError for our custom error type to use with Actix
impl ResponseError for ColorMixerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ColorMixerError::InvalidColorFormat(_) | ColorMixerError::UnsupportedColor(_) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: self.to_string(),
                })
            }
            ColorMixerError::MaxColorsReached => HttpResponse::BadRequest().json(ErrorResponse {
                error: self.to_string(),
            }),
            ColorMixerError::NoColors => HttpResponse::BadRequest().json(ErrorResponse {
                error: "No colors in mixer".to_string(),
            }),
            _ => HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            }),
        }
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
    data: web::Data<Arc<RwLock<ColorMixer>>>,
    req: web::Json<AddColorRequest>,
) -> Result<HttpResponse, ActixError> {
    // Validate the request
    req.validate().map_err(|e| {
        error!("Validation error: {}", e);
        ColorMixerError::InvalidColorFormat(e.to_string())
    })?;

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
    Ok(HttpResponse::Ok().json(SuccessResponse {
        success: true,
        data: ColorResponse {
            color: color.to_hex(),
            rgb: color.rgb(),
        },
    }))
}

// Handler to get the current color
async fn get_current_color(
    data: web::Data<Arc<RwLock<ColorMixer>>>,
) -> Result<HttpResponse, ActixError> {
    // Access the color mixer with write lock since get_mixed_color requires mutable access
    let mut mixer = data.write().map_err(|e| {
        error!("Failed to acquire write lock: {}", e);
        ColorMixerError::InternalError(format!("Lock error: {}", e))
    })?;

    // Get the mixed color
    let color = mixer.get_mixed_color()?;
    
    // Return the mixed color
    Ok(HttpResponse::Ok().json(SuccessResponse {
        success: true,
        data: ColorResponse {
            color: color.to_hex(),
            rgb: color.rgb(),
        },
    }))
}

// Handler to clear all colors
async fn clear_colors(
    data: web::Data<Arc<RwLock<ColorMixer>>>,
) -> Result<HttpResponse, ActixError> {
    // Access the color mixer with write lock since we're modifying it
    let mut mixer = data.write().map_err(|e| {
        error!("Failed to acquire write lock: {}", e);
        ColorMixerError::InternalError(format!("Lock error: {}", e))
    })?;

    // Clear the colors
    mixer.clear();
    
    // Return success
    Ok(HttpResponse::Ok().json(SuccessResponse {
        success: true,
        data: "Colors cleared",
    }))
}

// Handler to get the mixing history
async fn get_history(
    data: web::Data<Arc<RwLock<ColorMixer>>>,
) -> Result<HttpResponse, ActixError> {
    // Access the color mixer with read lock since we're only reading
    let mixer = data.read().map_err(|e| {
        error!("Failed to acquire read lock: {}", e);
        ColorMixerError::InternalError(format!("Lock error: {}", e))
    })?;

    // Get the history
    let history = mixer.get_history();
    
    // Return the history
    Ok(HttpResponse::Ok().json(SuccessResponse {
        success: true,
        data: history,
    }))
}

// Status handler
async fn status() -> impl Responder {
    HttpResponse::Ok().json(SuccessResponse {
        success: true,
        data: "Color Mixer API is running",
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set up logging");
    
    info!("Starting Color Mixer API");

    // Create the shared state with proper Arc wrapper
    let color_mixer = web::Data::new(Arc::new(RwLock::new(ColorMixer::new())));

    // Start the HTTP server
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allowed_headers(vec!["Content-Type"])
            .max_age(3600);

        App::new()
            .wrap(middleware::Logger::default()) // Enable logging
            .wrap(cors) // Enable CORS
            .app_data(color_mixer.clone())
            // API routes
            .service(
                web::scope("/api")
                    .route("/status", web::get().to(status))
                    .route("/add-color", web::post().to(add_color))
                    .route("/current-color", web::get().to(get_current_color))
                    .route("/clear", web::delete().to(clear_colors))
                    .route("/history", web::get().to(get_history))
            )
            // Static files
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
