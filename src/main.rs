#[macro_use]
extern crate diesel;

use structopt::StructOpt;
use dotenv;
use diesel::prelude::*;

mod sql_types;
mod download;
mod db;
mod models;
mod schema;
mod cli;

#[actix_rt::main]
async fn main() {
    let opt = cli::Opt::from_args();
    dotenv::dotenv().ok();
    let conn = db::establish_connection();

    match opt {
        cli::Opt::Download { url } => {
            println!("arg {}", url);
            let hash = if let Ok(hash) = download::ytdl(&url).await {
                hash
            } else {
                println!("youtube-dl failed, using readability fallback...");
                download::readability(&url).await.unwrap()
            };
            let file = models::File {
                title: "Placeholder".into(),
                url,
                hash: hash.clone(),
                file_type: sql_types::FileType::Video
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
        }
    }
}
