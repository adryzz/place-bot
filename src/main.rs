#[macro_use]
extern crate enum_display_derive;

use std::time::Duration;

use pixel::Color;
use reqwest::Client;

mod pixel;

#[tokio::main]
async fn main() {
    let config_json = tokio::fs::read("config.json").await.expect("Couldn't find config file");
    let tokens: Vec<String> = serde_json::from_slice(&config_json).expect("Invalid configuration");

    let mut client = Client::builder()
    .build().unwrap();
    place_task(0, &mut client, 53, 3, Color::Black, &tokens[0]).await
}


async fn place_task(id: i32, client: &mut Client, x: i32, y: i32, color: Color, bearer: &str) {
    match pixel::make_query(client, x, y, color, bearer).await {
        Err(e) => {
            eprintln!("[{}] {}", id, e)
        }
        Ok(_) => {
            println!("[{}] Placed a {} pixel at ({}, {})", id, color, x, y);
        }
    }

    println!("[{}] Waiting cooldown...", id);
    // Wait for 5 minutes
    tokio::time::sleep(Duration::from_secs(60 * 5)).await
}