use crate::models::File;
use crate::sql_types::FileType;
use anyhow::{Context, Result};
use crossbeam::channel::{unbounded, Receiver, Sender};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use duct::cmd;
use std::env;
use std::thread;

use crate::schema;
use crate::web::DbPool;

pub enum Msg {
    Download(String),
}

pub fn start_download_thread() -> Sender<Msg> {
    let (sender, receiver) = unbounded::<Msg>();

    thread::spawn(|| {
        handle_messages(receiver);
    });

    sender
}

pub fn handle_messages(receiver: Receiver<Msg>) {
    let db_url = env::var("DATABASE_URL").expect("Please set DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool");

    while let Ok(msg) = receiver.recv() {
        match msg {
            Msg::Download(url) => download(&pool, url),
        };
    }
}

pub fn download(pool: &DbPool, url: String) -> Result<()> {
    save_file(pool, ytdl(&url)?)?;
    Ok(())
}

pub fn save_file(pool: &DbPool, file: File) -> Result<()> {
    let conn = pool.get().context("db connection")?;
    diesel::insert_into(schema::files::table)
        .values(&file)
        .execute(&conn)
        .map_err(|_| anyhow!("save db entry"))?;
    Ok(())
}

pub fn ytdl(url: &String) -> Result<File> {
    let hash = cmd!("youtube-dl", "-o", "-", url)
        .pipe(cmd!("ipfs", "add", "--quiet", "--"))
        .read()
        .context("failed to run command")?;

    Ok(File {
        hash: hash.clone(),
        url: url.into(),
        title: "unknown title".into(),
        file_type: FileType::Video,
    })
}
