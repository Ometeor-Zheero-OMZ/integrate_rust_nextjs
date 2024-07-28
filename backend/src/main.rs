use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct HealthcheckResponse {
    status: String,
    message: String,
}

#[get("/healthchecker")]
async fn healthchecker() -> impl Responder {
    let response = HealthcheckResponse {
        status: "success".to_string(),
        message: "Hello from Rust!".to_string(),
    };

    web::Json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::CONTENT_TYPE])
                    .max_age(3600)
            )
            .service(healthchecker)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
