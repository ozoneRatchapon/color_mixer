use actix_files as fs;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::Deserialize;
use std::sync::Mutex;
mod color_mixer;

use color_mixer::{Color, ColorMixer};

#[derive(Deserialize)]
struct ColorRequest {
    color: String,
}

async fn add_color(
    data: web::Data<Mutex<ColorMixer>>,
    req: web::Json<ColorRequest>,
) -> impl Responder {
    let color = match req.color.as_str() {
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        _ => return HttpResponse::BadRequest().body("Invalid color"),
    };

    let mut mixer = data.lock().unwrap();
    mixer.add_color(color);

    let mixed_color = mixer.get_mixed_color();
    HttpResponse::Ok().json(serde_json::json!({ "color": mixed_color }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let color_mixer = web::Data::new(Mutex::new(ColorMixer::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(color_mixer.clone())
            .service(web::resource("/add_color").route(web::post().to(add_color)))
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
