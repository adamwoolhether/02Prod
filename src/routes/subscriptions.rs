use actix_web::{web, App, HttpResponse, HttpServer};
use sqlx::PgConnection;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<FormData>, _connection: web::Data<PgConnection>) -> HttpResponse {
    
    HttpResponse::Ok().finish()
}