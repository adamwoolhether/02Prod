use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

/// Mark `run` as public.
/// No longer being a binary entrypoint, we can mark it
/// as async without having ot use any proc-marco incantation.
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}