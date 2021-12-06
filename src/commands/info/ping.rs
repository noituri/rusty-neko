use crate::traits::command_trait::Command;
use serenity::async_trait;
use crate::structures::arg::Arg;
use crate::structures::args::{Args, ArgsTrait};
use crate::structures::bot::{Bot};
use serenity::{client::Context, model::channel::Message};
use std::error::Error;

pub struct PingCommand; 

#[async_trait]
impl Command for PingCommand {
    fn name(&self) -> &str {
        "ping"
    }

    fn args(&self) -> Vec<Arg> {
        return vec![]
    }

    fn category(&self) -> &str {
        "info"
    }

    fn is_owner_only(&self) -> bool {
        false
    }

    async fn execute(&self, bot: &Bot, ctx: &Context, msg: &Message, args: &Args) -> Result<(), Box<dyn Error + Send + Sync>> {
        msg.channel_id.send_message(
            ctx,
            | m | {
                m.content(
                    format!(
                        "Command ran with {} arguments.",
                        args.size()
                    )
                )
            }
        ).await;

        Ok(())
    }
}