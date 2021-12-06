use crate::traits::command_trait::Command;
use serenity::async_trait;
use crate::structures::arg::Arg;
use crate::structures::args::{Args, ArgsTrait};
use crate::structures::bot::{Bot};
use serenity::{client::Context, model::channel::Message};
use std::error::Error;
use crate::enums::raw_arg_types::raw_arg_types;

pub struct PingCommand; 

#[async_trait]
impl Command for PingCommand {
    fn name(&self) -> &str {
        "ping"
    }

    fn args(&self) -> Vec<Arg> {
        return vec![
            Arg {
                name: "test arg".to_string(),
                description: "this is a test argument.".to_string(),
                required: true,
                example: "10".to_string(),
                regexes: vec![],
                expect: raw_arg_types::Integer,
                min_len: 0,
                max_len: 20,
                rest: false
            }
        ]
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