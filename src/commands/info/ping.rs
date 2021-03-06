use crate::enums::raw_arg_types::RawArgTypes;
use crate::structures::arg::Arg;
use crate::structures::args::{Args, ArgsTrait};
use crate::structures::bot::Bot;
use crate::structures::extras::Extras;
use crate::traits::command_trait::Command;
use serenity::async_trait;
use serenity::{client::Context, model::channel::Message};
use std::error::Error;

pub struct PingCommand;

#[async_trait]
impl Command for PingCommand {
    fn name(&self) -> String {
        "ping".to_owned()
    }

    fn args(&self) -> Vec<Arg> {
        return vec![
            Arg {
                name: "test arg".to_string(),
                description: "this is a test argument.".to_string(),
                required: false,
                example: "10".to_string(),
                regexes: vec![],
                expect: RawArgTypes::Integer,
                min_len: 0,
                max_len: 20,
            },
            Arg {
                name: "bool".to_string(),
                description: "this is a test argument.".to_string(),
                required: false,
                example: "true".to_string(),
                regexes: vec![],
                expect: RawArgTypes::Bool,
                min_len: 0,
                max_len: 0,
            },
            Arg {
                name: "target".to_string(),
                description: "this is a test argument.".to_string(),
                required: false,
                example: "tmr".to_string(),
                regexes: vec![],
                expect: RawArgTypes::String,
                min_len: 0,
                max_len: 0,
            },
        ];
    }

    fn staff_only(&self) -> bool {
        true
    }

    fn category(&self) -> String {
        "info".to_owned()
    }

    async fn execute(
        &self,
        _bot: &Bot,
        ctx: &Context,
        msg: &Message,
        args: &Args,
        _extras: &Extras,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = msg
            .channel_id
            .send_message(ctx, |m| {
                m.content(format!("Command ran with {} arguments.", args.size()))
            })
            .await;

        Ok(())
    }
}
