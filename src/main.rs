use actix_web::client::Client;

#[actix_rt::main]
async fn main() {
    let mut client = Client::default();

    let response = client.get("https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html")
        .send()
        .await;

    println!("Response: {:?}", response);
}
