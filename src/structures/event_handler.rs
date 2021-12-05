use crate::structures::bot::Bot;
use serenity::{async_trait, client::{EventHandler, Context}, model::prelude::Ready};
use serenity::model::channel::Message;
use crate::events::*;

pub struct State {
    bot: Bot
}

impl State {
    pub async fn new() -> Self {
        Self {
            bot: Bot::new()
        }
    }
}

#[async_trait]
impl EventHandler for State {
    async fn ready(&self, ctx: Context, data: Ready) {
        ready::ready(&self.bot, ctx, data)
    }

    async fn message(&self, ctx: Context, new_message: Message) {
        message_create::message_create(&self.bot, ctx, new_message).await;
    }
}