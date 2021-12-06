use crate::structures::args::Args;
use crate::structures::bot::{Bot};
use serenity::{async_trait, client::Context, model::channel::Message};
use std::error::Error;
use crate::structures::arg::Arg;
use crate::structures::extras::Extras;

#[async_trait]
pub trait Command: Send + Sync {
    fn name(&self) -> String;

    fn category(&self) -> String;

    fn args(&self) -> Vec<Arg> {
        return vec![]
    }

    fn is_owner_only(&self) -> bool {
        return false
    }

    async fn execute(
        &self,
        bot: &Bot,
        ctx: &Context,
        msg: &Message,
        args: &Args,
        extras: &Extras
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}