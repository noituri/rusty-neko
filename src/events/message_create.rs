use crate::handlers::*;
use crate::structures::bot::Bot;
use serenity::{client::Context, model::channel::Message};

pub async fn message_create(bot: &Bot, ctx: Context, msg: Message) {
    let _ = command_handler::command_handler(bot, &ctx, &msg).await;
}
