use anyhow::{Context, Result};
use duct::cmd;
use crate::models::File;
use crate::sql_types::FileType;

pub async fn ytdl(url: &str) -> Result<File> {
    let hash = cmd!("youtube-dl", "-o", "-", url)
        .pipe(cmd!("ipfs", "add", "--quiet", "--"))
        .read()
        .context("failed to run command")?;

    Ok(File {
        hash: hash.clone(),
        url: url.into(),
        title: "placeholder".into(),
        file_type: FileType::Video,
    })
}
