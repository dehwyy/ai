use serenity::all::GatewayIntents;
use serenity::client::Client;
use songbird::SerenityInit;

mod config;
mod event_handler;
mod framework;
mod logger;
mod static_clients;
mod utils;

use utils::AnyResult;

#[tokio::main]
async fn main() -> AnyResult {
    let config = config::Config::new();
    logger::Logger::init(logger::LoggerConfig::default());

    let mut client = Client::builder(config.bot_token, GatewayIntents::all())
        .framework(framework::new())
        .event_handler(event_handler::EventHandler::new())
        .register_songbird()
        .await?;

    if let Err(err) = client.start_autosharded().await {
        println!("Client error: {}", err);
    }

    Ok(())
}
