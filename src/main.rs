#[macro_use]
extern crate enum_display_derive;
use deadpool::unmanaged;
use pixel::Color;
use reqwest::Client;
use std::time::Duration;

mod pixel;

const POSITION: (i32, i32) = (300, 180);

const PIXELS: [(i32, i32, Color); 15] = [
    (0, 0, Color::LightBlue),
    (0, 1, Color::LightPink),
    (0, 2, Color::White),
    (0, 3, Color::LightPink),
    (1, 4, Color::LightBlue),
    (1, 0, Color::LightBlue),
    (1, 1, Color::LightPink),
    (1, 2, Color::White),
    (1, 3, Color::LightPink),
    (1, 4, Color::LightBlue),
    (2, 0, Color::LightBlue),
    (2, 1, Color::LightPink),
    (2, 2, Color::White),
    (2, 3, Color::LightPink),
    (2, 4, Color::LightBlue),
];

#[tokio::main]
async fn main() {
    let config_json = tokio::fs::read("config.json")
        .await
        .expect("Couldn't find config file");
    let tokens: Vec<String> = serde_json::from_slice(&config_json).expect("Invalid configuration");

    let mut accounts = Vec::with_capacity(tokens.len());
    for i in 0..tokens.len() {
        accounts.push(Account {
            token: tokens[i].clone(),
            client: Client::new(),
            id: i,
        })
    }

    println!("Created account pool of size {}", accounts.len());

    let pool = AccountPool::from(accounts);

    for pix in PIXELS {
        let account = pool.get().await.unwrap();
        tokio::spawn(async move {
            account.place_task(pix.0 + POSITION.0, pix.1 + POSITION.1, pix.2).await;
        });
        
    }
}

type AccountPool = unmanaged::Pool<Account>;

#[derive(Debug, Clone)]
struct Account {
    token: String,
    client: Client,
    id: usize,
}

impl Account {
    async fn place_task(&self, x: i32, y: i32, color: Color) {
        match pixel::make_query(&self.client, x, y, color, &self.token).await {
            Err(e) => {
                eprintln!("[{}] {}", self.id, e)
            }
            Ok(_) => {
                println!("[{}] Placed a {} pixel at ({}, {})", self.id, color, x, y);
            }
        }

        println!("[{}] Waiting cooldown...", self.id);
        // Wait for 5 minutes
        tokio::time::sleep(Duration::from_secs(60 * 5)).await
    }
}
