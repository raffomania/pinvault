use std::env;

mod download;

#[actix_rt::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("arg {}", args[1]);
        let hash = if let Ok(hash) = download::ytdl(&args[1]).await {
            hash
        } else {
            println!("youtube-dl failed, using readability fallback...");
            download::readability(&args[1]).await.unwrap()
        };
        println!("hash: {}", hash);
    } else {
        println!("please provide an url");
    }
}
