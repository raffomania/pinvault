use actix_web::{guard, middleware, web, App, HttpResponse, HttpServer, Responder};
use anyhow::{Context, Result};
use askama::Template;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Internal Server Error")]
struct AppError(#[from] anyhow::Error);

impl actix_web::error::ResponseError for AppError {}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

async fn index() -> impl Responder {
    Index
}

async fn p404() -> impl Responder {
    "Not found"
}

pub async fn start_server() -> Result<()> {
    HttpServer::new(|| {
        App::new()
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
    .bind("127.0.0.1:8000")?
    .run()
    .await
    .context("Failed to start server")
}
