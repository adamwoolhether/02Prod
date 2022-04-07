use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // actix requires our pg conn to be cloneable, we use the `web::Data` extractor,
    // which wraps our connection in an ARC pointer.
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // use actix-web `app_data` method to attach stateful deps,
            // the cloned connection in this case.
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
