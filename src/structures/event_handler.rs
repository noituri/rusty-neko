use crate::structures::bot::Bot;
use serenity::{async_trait, client::{EventHandler, Context}, model::prelude::Ready};
use serenity::model::channel::Message;
use serenity::model::interactions::Interaction;
use crate::events::*;
use crate::events::interaction_create::interaction_create;

pub struct State {
    bot: Bot
}

impl State {
    pub async fn new() -> Self {
        Self {
            bot: Bot::new().await
        }
    }
}

#[async_trait]
impl EventHandler for State {
    async fn message(&self, ctx: Context, new_message: Message) {
        message_create::message_create(&self.bot, ctx, new_message).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        interaction_create(&self.bot, &ctx, interaction).await;
    }

    async fn ready(&self, ctx: Context, data: Ready) {
        ready::ready(&self.bot, ctx, data)
    }
}