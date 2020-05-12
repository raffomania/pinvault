use crate::models::File;
use crate::sql_types::FileType;
use anyhow::{Context, Result};
use crossbeam::channel::{unbounded, Receiver, Sender};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use duct::cmd;
use std::collections::HashMap;
use std::env;
use std::thread;
use uuid::Uuid;

use crate::schema;
use crate::web::DbPool;

pub enum Msg {
    Download(String),
    RequestInfo,
}

pub enum Info {
    Downloads(Vec<Download>),
}

#[derive(Clone)]
pub struct Download {
    url: String,
    finished: bool,
    error: Option<String>,
}

pub fn start_download_thread() -> (Sender<Msg>, Receiver<Info>) {
    let (req_sender, req_receiver) = unbounded::<Msg>();
    let (info_sender, info_receiver) = unbounded::<Info>();

    thread::spawn(|| {
        handle_messages(info_sender, req_receiver);
    });

    (req_sender, info_receiver)
}

pub fn handle_messages(sender: Sender<Info>, receiver: Receiver<Msg>) {
    let db_url = env::var("DATABASE_URL").expect("Please set DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool");
    let mut downloads: HashMap<Uuid, Download> = HashMap::new();

    while let Ok(msg) = receiver.recv() {
        match msg {
            Msg::Download(url) => {
                download(&pool, &mut downloads, url);
            }
            Msg::RequestInfo => {
                let downloads: Vec<Download> = downloads.values().cloned().collect();
                sender.send(Info::Downloads(downloads));
            }
        };
    }
}

pub fn download(pool: &DbPool, downloads: &mut HashMap<Uuid, Download>, url: String) -> Result<()> {
    let id = Uuid::new_v4();
    let mut download = Download {
        url: url.clone(),
        finished: false,
        error: None,
    };
    downloads.insert(id, download.clone());
    save_file(pool, ytdl(&url)?)?;
    download.finished = true;
    downloads.insert(id, download);
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
