use actix_files;
use actix_web::{guard, http, middleware, web, App, HttpResponse, HttpServer, Responder};
use anyhow::{self, Context};
use askama::Template;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use serde::Deserialize;
use std::env;
use thiserror::Error;

use crate::download;
use crate::models::File;
use crate::schema;
use crate::sql_types::FileType;

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
    let files = web::block(move || schema::files::table.load::<File>(&conn))
        .await
        .map_err(|_| anyhow!("load user"))?;
    Ok(Index { files })
}

#[derive(Deserialize)]
struct AddQueryParams {
    url: String,
    title: String,
}

async fn add_file(
    pool: web::Data<DbPool>,
    web::Query(params): web::Query<AddQueryParams>,
) -> Result<impl Responder, AppError> {
    let file = download::ytdl(&params.url, Some(params.title)).await?;
    let conn = pool.get().context("db connection")?;
    web::block(move || {
        diesel::insert_into(schema::files::table)
            .values(&file)
            .execute(&conn)
    })
    .await
    .map_err(|_| anyhow!("save db entry"))?;

    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, "/added")
        .finish())
}

#[derive(Template)]
#[template(path = "added.html")]
struct AddedTemplate;

async fn added_file() -> impl Responder {
    AddedTemplate
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
            .service(web::resource("/add").route(web::get().to(add_file)))
            .service(web::resource("/added").route(web::get().to(added_file)))
            .service(actix_files::Files::new("/static", "static").show_files_listing())
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
