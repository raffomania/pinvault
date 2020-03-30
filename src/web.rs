use actix_web::{guard, middleware, web, App, HttpResponse, HttpServer, Responder};
use anyhow::{Context, self};
use askama::Template;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
use thiserror::Error;

use crate::schema;
use crate::models::File;

#[derive(Debug, Error)]
#[error("Internal Server Error")]
struct AppError(#[from] anyhow::Error);

impl actix_web::error::ResponseError for AppError {}

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    files: Vec<File>,
}

async fn index(pool: web::Data<DbPool>) -> Result<impl Responder, AppError> {
    let conn = pool.get().context("db connection")?;
    let files = web::block(move || {
        schema::files::table
            .load::<File>(&conn)
    }).await
      .map_err(|_e| anyhow!("load user"))?;
    Ok(Index { files })
}

async fn p404() -> impl Responder {
    "Not found"
}

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
            .service(web::resource("/").route(web::get().to(index)))
            .default_service(
                web::resource("").route(web::get().to(p404)).route(
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
