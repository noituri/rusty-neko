use serenity::model::channel::Message;
use serenity::client::Context;
use crate::structures::args::Args;
use crate::structures::bot::Bot;
use serenity::model::channel::Channel;
use crate::config;
use crate::functions::command_permissions_for::command_permissions_for;
use crate::functions::find_command;
use crate::functions::parse_args::parse_args;
use crate::structures::extras::Extras;

pub async fn command_handler(bot: &Bot, ctx: &Context, msg: &Message) {
    if msg.author.bot {
        return;
    }

    let ch: Channel = ctx.cache.channel(msg.channel_id).await.unwrap();

    if ch.guild().is_none() {
        return;
    }

    let mut prefix: &str = "";

    for pr in config::PREFIXES {
        if msg.content.starts_with(pr) {
            prefix = pr;
            break;
        }
    }

    if prefix.len() == 0 {
        return;
    }

    let mut raw_args: Vec<&str> = msg.content[prefix.len()..].trim().split(" ").collect();

    let cmd = raw_args.remove(0).to_lowercase();

    let command = find_command::find_command(cmd.as_str());

    match command {
        Ok(command) => {
            let extras = Extras {
                prefix: prefix.to_string(),
                command_string: cmd.to_string()
            };

            let perms = command_permissions_for(bot, &command, ctx, msg, &extras, true).await.unwrap();

            if !perms {
                return;
            }

            let parsed_args = parse_args(bot, ctx, msg, &command, raw_args, &extras).await;

            if parsed_args.is_err() {
                return;
            }

            let args = Args {
                list: parsed_args.unwrap()
            };

            let res = command.execute(bot, ctx, msg, &args, &extras).await;

            if res.is_err() {
                let err = res.unwrap_err();

                let _ = msg.channel_id.send_message(ctx, | m | {
                    m.content(
                        format!(
                            "An error occurred while trying to execute `{}`: ```\n{}```",
                            command.name(),
                            err
                        )
                    )
                }).await;
            }
        }

        Err(_) => {
            println!(
                "{}",
                format!(
                    "{} tried to run command {} but it does not exist.",
                    msg.author.tag(),
                    cmd 
                )
            )
        }
    }
}