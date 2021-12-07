use serenity::client::{bridge::gateway::GatewayIntents, Client};

use structures::event_handler::State;

mod commands;
mod config;
mod enums;
mod events;
mod functions;
mod handlers;
mod structures;
mod traits;
mod util;

#[tokio::main]
async fn main() {
    let mut client = Client::builder(config::TOKEN)
        .event_handler(State::new().await)
        .intents(GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES)
        .application_id(877283475894444072)
        .await
        .expect("Failed to create client.");

    if let Err(why) = client.start().await {
        println!("Failed to start bot, {:?}", why);
    }
}
