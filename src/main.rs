use actix_web::client::{Client, SendRequestError};
use bytes::buf::BufExt;
use readability::extractor;
use url::Url;
use std::{env,
          io::Cursor,
          process::{Command, Stdio}};
use anyhow::{Result, anyhow, Context};
use ipfs_api::IpfsClient;

#[actix_rt::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("arg {}", args[1]);
        println!("hash: {}", download_video(&args[1]).await.unwrap());
        // scrape(&args[1]).await;
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

async fn download_video(url: &String) -> Result<String> {
    let ipfs = IpfsClient::default();

    let child = Command::new("youtube-dl")
        .args(&["-o", "-"])
        .arg(url)
        .stdout(Stdio::piped())
        .spawn()?;

    match ipfs.add(child.stdout.ok_or(anyhow!("couldn't get ytdl stdout"))?).await {
        Ok(res) => Ok(res.hash),
        Err(e) => {
            println!("{:?}", e);
            Err(anyhow!("couldn't add to ipfs"))
        }
    }
}
