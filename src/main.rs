use std::env;

mod download;

#[actix_rt::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("arg {}", args[1]);
        println!("hash: {}", download::ytdl(&args[1]).await.unwrap());
    } else {
        println!("please provide an url");
    }
}
