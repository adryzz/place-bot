use reqwest::Client;

mod pixel;

#[tokio::main]
async fn main() {
    let mut client = Client::builder()
    .build().unwrap();
    pixel::make_query(&mut client, 53, 3, 27, "your-bearer").await.unwrap()
}
