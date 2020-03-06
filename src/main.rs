use actix_web::client::{Client, SendRequestError};
use bytes::buf::BufExt;
use readability::extractor;
use url::Url;
use std::env;

#[actix_rt::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("arg {}", args[1]);
        scrape(&args[1]).await;
    } else {
        println!("please provide an url");
    }
}

async fn scrape(url: &String) -> Result<(), SendRequestError> {
    let client = Client::default();

    let mut response = client.get(url)
        .send()
        .await?;

    let status = response.status();
    println!("status {}", status);

    let body = response.body().await.unwrap();

    let extracted = extractor::extract(&mut body.reader(), &Url::parse(url).unwrap()).unwrap();

    println!("{:?}", extracted.text);
    Ok(())
}
