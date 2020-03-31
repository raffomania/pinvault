use actix_files;
use actix_web::{guard, middleware, web, App, HttpResponse, HttpServer};
use anyhow::{self};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
use thiserror::Error;

mod filters;
mod handlers;

#[derive(Debug, Error)]
#[error("Internal Server Error")]
pub struct AppError(#[from] anyhow::Error);

impl actix_web::error::ResponseError for AppError {}

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub async fn start_server() {
    let db_url = env::var("DATABASE_URL").expect("Please set DATABASE_URL");

    let manager = ConnectionManager::<SqliteConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(handlers::index)
            .service(handlers::hash_file)
            .service(handlers::add_file)
            .service(handlers::added_file)
            .service(actix_files::Files::new("/static", "static").show_files_listing())
            .default_service(
                web::resource("")
                    .route(web::get().to(handlers::p404))
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    })
    .bind("127.0.0.1:8000")
    .expect("failed to open server socket")
    .run()
    .await
    .expect("Failed to start server")
}
