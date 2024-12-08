use reqwest::Proxy;
use serenity::all::{EventHandler, GatewayIntents};
use serenity::async_trait;
use serenity::client::Client;
use songbird::SerenityInit;

mod config;
mod framework;

struct H;

#[async_trait]
impl EventHandler for H {
    async fn ready(&self, ctx: serenity::client::Context, _: serenity::model::gateway::Ready) {
        println!("{} is connected!", ctx.cache.current_user().name);
    }
}

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let mut client = Client::builder(config.bot_token, GatewayIntents::all())
        .framework(framework::new())
        .event_handler(H)
        .register_songbird()
        .await
        .unwrap();

    if let Err(err) = client.start().await {
        println!("Client error: {}", err);
    }
}
