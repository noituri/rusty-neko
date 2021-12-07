use crate::enums::staff_roles::StaffRoles;
use crate::structures::arg::Arg;
use crate::structures::args::Args;
use crate::structures::bot::Bot;
use crate::structures::extras::Extras;
use serenity::{async_trait, client::Context, model::channel::Message};
use std::error::Error;

#[async_trait]
pub trait Command: Send + Sync {
    fn name(&self) -> String;

    fn category(&self) -> String;

    fn args(&self) -> Vec<Arg> {
        vec![]
    }

    fn staff_only(&self) -> bool {
        false
    }

    fn staff_roles(&self) -> Vec<StaffRoles> {
        vec![]
    }

    fn owner_only(&self) -> bool {
        false
    }

    async fn execute(
        &self,
        bot: &Bot,
        ctx: &Context,
        msg: &Message,
        args: &Args,
        extras: &Extras,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}
