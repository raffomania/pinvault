#[macro_use]
extern crate diesel;

use structopt::StructOpt;
use dotenv;
use diesel::prelude::*;

mod download;
mod db;
mod models;
mod schema;
mod cli;

#[actix_rt::main]
async fn main() {
    let opt = cli::Opt::from_args();
    dotenv::dotenv().ok();

    match opt {
        cli::Opt::Download { url } => {
            println!("arg {}", url);
            let hash = if let Ok(hash) = download::ytdl(&url).await {
                hash
            } else {
                println!("youtube-dl failed, using readability fallback...");
                download::readability(&url).await.unwrap()
            };
            println!("hash: {}", hash);
        },
        cli::Opt::List => {
            let connection = db::establish_connection();
            let results = schema::files::table
                .load::<models::File>(&connection)
                .expect("Error loading files");
            for file in results {
                println!("{} - http://localhost:8080/ipfs/{}", file.title, file.hash);
            }
        }
    }
}
