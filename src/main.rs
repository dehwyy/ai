use api::endpoints::{init_endpoints, Endpoints};
use serenity::all::GatewayIntents;
use serenity::client::Client;
use songbird::SerenityInit;

mod api;
mod config;
mod event_handler;
mod framework;
mod libs;
mod logger;
mod utils;

use utils::AnyResult;

#[tokio::main]
async fn main() -> AnyResult {
    let config = config::Config::new();

    logger::Logger::init(logger::LoggerConfig::default());
    init_endpoints(Endpoints {
        tts: config.tts_api_url,
    });

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
