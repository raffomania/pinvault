#[macro_use]
extern crate diesel;

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate actix_web;

use diesel::prelude::*;
use dotenv;
use structopt::StructOpt;

mod cli;
mod db;
mod download;
mod models;
mod schema;
mod sql_types;
mod web;

#[actix_rt::main]
async fn main() {
    let opt = cli::Opt::from_args();
    dotenv::dotenv().ok();
    let conn = db::establish_connection();

    match opt {
        cli::Opt::Download { url } => {
            println!("downloading {}", url);
            let file = download::ytdl(&url, None)
                .await
                .expect("Error downloading file");
            diesel::insert_into(schema::files::table)
                .values(&file)
                .execute(&conn)
                .expect("Error saving file to DB");
            println!(
                "Downloaded successfully! view file at http://localhost:8080/ipfs/{}",
                file.hash
            );
        }
        cli::Opt::List => {
            let results = schema::files::table
                .load::<models::File>(&conn)
                .expect("Error loading files");
            for file in results {
                println!("{} - http://localhost:8080/ipfs/{}", file.title, file.hash);
            }
        }
        cli::Opt::Server => {
            web::start_server().await;
        }
    }
}
