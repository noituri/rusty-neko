use serenity::client::{
    Client, bridge::gateway::GatewayIntents
};

use structures::{
    event_handler::State
};

mod traits;
mod util;
mod structures;
mod config;
mod commands;
mod handlers;
mod enums;
mod events;
mod functions;

#[tokio::main]
async fn main() {
    let mut client = Client::builder(config::TOKEN)
        .event_handler(State::new().await)
        .intents(
            GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES
        )
        .await
        .expect("Failed to create client.");

    if let Err(why) = client.start().await {
        println!("Failed to start bot, {:?}", why);
    }
}

