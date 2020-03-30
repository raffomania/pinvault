#[macro_use]
extern crate diesel;

#[macro_use]
extern crate anyhow;

use structopt::StructOpt;
use dotenv;
use diesel::prelude::*;

mod sql_types;
mod download;
mod db;
mod models;
mod schema;
mod cli;
mod web;

#[actix_rt::main]
async fn main() {
    let opt = cli::Opt::from_args();
    dotenv::dotenv().ok();
    let conn = db::establish_connection();

    match opt {
        cli::Opt::Download { url } => {
            println!("arg {}", url);
            let (hash, file_type) = if let Ok(hash) = download::ytdl(&url).await {
                (hash, sql_types::FileType::Video)
            } else {
                println!("youtube-dl failed, using readability fallback...");
                (download::readability(&url).await.unwrap(), sql_types::FileType::Text)
            };
            let file = models::File {
                hash: hash.clone(),
                url,
                title: "placeholder".into(),
                file_type: file_type
            };

            diesel::insert_into(schema::files::table)
                .values(&file)
                .execute(&conn)
                .expect("Error saving file to DB");
            println!("Downloaded successfully! view file at http://localhost:8080/ipfs/{}", hash);
        },
        cli::Opt::List => {
            let results = schema::files::table
                .load::<models::File>(&conn)
                .expect("Error loading files");
            for file in results {
                println!("{} - http://localhost:8080/ipfs/{}", file.title, file.hash);
            }
        },
        cli::Opt::Server => {
            web::start_server().await;
        }
    }
}
