use actix_web::client::Client;
use anyhow::{anyhow, Context, Result};
use bytes::buf::BufExt;
use duct::cmd;
use readability::extractor;
use url::Url;

pub async fn ytdl(url: &str) -> Result<String> {
    cmd!("youtube-dl", "-o", "-", url)
        .pipe(cmd!("ipfs", "add", "--quiet", "--"))
        .read()
        .context("failed to run command")
}

pub async fn readability(url: &str) -> Result<String> {
    let client = Client::default();

    let mut response = client
        .get(url)
        .send()
        .await
        .map_err(|_| anyhow!("couldn't fetch page"))?;

    let status = response.status();
    println!("status {}", status);

    let body = response.body().await.unwrap();

    let extracted = extractor::extract(&mut body.reader(), &Url::parse(url).unwrap()).unwrap();

    cmd!("ipfs", "add", "--quiet", "--")
        .stdin_bytes(extracted.text)
        .read()
        .context("failed to add to ipfs")
}
