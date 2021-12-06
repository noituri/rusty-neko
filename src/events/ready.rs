use crate::structures::bot::{Bot};
use serenity::{client::Context};
use serenity::{model::prelude::Ready};

pub fn ready(_bot: &Bot, _ctx: Context, data: Ready) {
    println!(
        "{}",
        format!(
            "Ready on client {} and loaded {} commands.",
            data.user.tag(),
            Bot::commands().len()
        )
    );
}