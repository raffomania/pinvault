use actix_web::{http, web, HttpRequest, HttpResponse, Responder};
use anyhow::{self, Context};
use askama::Template;
use diesel::prelude::*;
use serde::Deserialize;
use std::env;

use super::filters;
use super::{AppError, DbPool};
use crate::download;
use crate::models::File;
use crate::schema;
use crate::sql_types::FileType;
use crossbeam::channel::Sender;

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    files: Vec<File>,
    req: HttpRequest,
    port: String,
}

#[get("/")]
pub async fn index(req: HttpRequest, pool: web::Data<DbPool>) -> Result<impl Responder, AppError> {
    let conn = pool.get().context("db connection")?;
    let files = web::block(move || schema::files::table.load::<File>(&conn))
        .await
        .map_err(|_| anyhow!("load user"))?;

    let port = env::var("PORT").unwrap_or_else(|_e| "8000".into());

    Ok(Index { files, req, port })
}

#[derive(Deserialize)]
pub struct AddQueryParams {
    url: String,
}

#[get("/add")]
pub async fn add_file(
    web::Query(params): web::Query<AddQueryParams>,
    downloader: web::Data<Sender<download::Msg>>,
) -> Result<impl Responder, AppError> {
    downloader
        .send(download::Msg::Download(params.url))
        .context("couldnt start download")?;
    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, "/added")
        .finish())
}

#[derive(Template)]
#[template(path = "added.html")]
struct AddedTemplate;

#[get("/added")]
pub async fn added_file() -> impl Responder {
    AddedTemplate
}

#[derive(Template)]
#[template(path = "file.html")]
struct FileTemplate {
    file: File,
    req: HttpRequest,
}

#[get("/file/hash/{hash}")]
pub async fn hash_file(
    pool: web::Data<DbPool>,
    path: web::Path<(String,)>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let conn = pool.get().context("db connection")?;
    let file = web::block(move || schema::files::table.find(&path.0).first(&conn))
        .await
        .map_err(|_| anyhow!("find file"))?;
    Ok(FileTemplate { file, req })
}

pub async fn p404() -> impl Responder {
    "Not found"
}
