use actix_web::client::{Client, SendRequestError};
use anyhow::{anyhow, Result};
use bytes::buf::BufExt;
use ipfs_api::IpfsClient;
use readability::extractor;
use std::{
    process::{Command, Stdio},
};
use url::Url;

pub async fn ytdl(url: &String) -> Result<String> {
    let ipfs = IpfsClient::default();

    let child = Command::new("youtube-dl")
        .args(&["-o", "-"])
        .arg(url)
        .stdout(Stdio::piped())
        .spawn()?;

    match ipfs
        .add(child.stdout.ok_or(anyhow!("couldn't get ytdl stdout"))?)
        .await
    {
        Ok(res) => Ok(res.hash),
        Err(e) => {
            println!("{:?}", e);
            Err(anyhow!("couldn't add to ipfs"))
        }
    }
}

pub async fn readability(url: &String) -> Result<(), SendRequestError> {
    let client = Client::default();

    let mut response = client.get(url).send().await?;

    let status = response.status();
    println!("status {}", status);

    let body = response.body().await.unwrap();

    let extracted = extractor::extract(&mut body.reader(), &Url::parse(url).unwrap()).unwrap();

    println!("{:?}", extracted.text);
    Ok(())
}
