use crate::structures::args::Args;
use crate::structures::bot::{Bot};
use serenity::{async_trait, client::Context, model::channel::Message};
use std::error::Error;
use crate::structures::arg::Arg;
use crate::structures::extras::Extras;

#[async_trait]
pub trait Command: Send + Sync {
    fn name(&self) -> &str;

    fn category(&self) -> &str;

    fn args(&self) -> Vec<Arg>;

    fn is_owner_only(&self) -> bool;

    async fn execute(
        &self,
        bot: &Bot,
        ctx: &Context,
        msg: &Message,
        args: &Args,
        extras: &Extras
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}