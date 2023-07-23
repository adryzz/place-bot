#[macro_use]
extern crate enum_display_derive;
use clap::Parser;
use deadpool::unmanaged;
use pixel::Color;
use rand::Rng;
use reqwest::{Client, ClientBuilder, Proxy};
use std::time::Duration;

mod pixel;
mod template;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let template = template::image_to_template(&args.template)
        .await
        .expect("Invalid template");

    println!("Loaded {} pixels from template", template.len());

    let config_json = tokio::fs::read(args.config)
        .await
        .expect("Couldn't find config file");
    let tokens: Vec<String> = serde_json::from_slice(&config_json).expect("Invalid configuration");

    let mut accounts = Vec::with_capacity(tokens.len());

    let client: Client;

    match args.proxy {
        Some(p) => {
            client = ClientBuilder::new()
                .proxy(Proxy::https(p).expect("Invalid proxy"))
                .build().unwrap();
        }
        None => {
            client = Client::new();
        }
    }

    for i in 0..tokens.len() {
        accounts.push(Account {
            token: tokens[i].clone(),
            client: client.clone(),
            id: i,
        })
    }

    println!("Created account pool of size {}", accounts.len());

    let pool = AccountPool::from(accounts);

    if args.r#loop {
        loop {
            place_template(template.clone(), &pool, args.x, args.y, args.randomize).await;
        }
    }

    place_template(template.clone(), &pool, args.x, args.y, args.randomize).await;
}

async fn place_template(template: Vec<(i32, i32, Color)>, pool: &AccountPool, x: i32, y: i32, random: u32) {
    for pix in template {
        let account = pool.get().await.unwrap();
        tokio::spawn(async move {
            account
                .place_task(pix.0 + x, pix.1 + y, pix.2, random)
                .await;
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
    async fn place_task(&self, x: i32, y: i32, color: Color, random: u32) {
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
        let r = rand::thread_rng().gen_range(0..(random+1));
        tokio::time::sleep(Duration::from_secs((60 * 5) + r as u64)).await
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Config file path
    #[arg(short, long)]
    config: String,

    /// Template file path
    #[arg(short, long)]
    template: String,

    /// Request proxy to use
    #[arg(short, long)]
    proxy: Option<String>,

    /// X offset
    #[arg(short, default_value_t = 0)]
    x: i32,

    /// Y offset
    #[arg(short, default_value_t = 0)]
    y: i32,

    /// Randomize the time offset between each request
    #[arg(short, long, default_value_t = 0)]
    randomize: u32,

    /// Keep drawing the same template even after you're finished
    #[arg(short, long, default_value_t = false)]
    r#loop: bool,
}
