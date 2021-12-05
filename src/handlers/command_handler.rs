use serenity::model::channel::Message;
use serenity::Client;
use serenity::client::Context;
use crate::structures::args::Args;
use crate::structures::bot::Bot;
use serenity::model::channel::Channel;
use crate::config;
use crate::functions::find_command;

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

    let raw_args: Vec<&str> = msg.content.trim()[prefix.len()..].split(" ").collect();

    let cmd: String = raw_args.get(0).unwrap().to_lowercase();

    let command = find_command::find_command(cmd.as_str());

    match command {
        Ok(command) => {
            let args: Args = Args {
                list: vec![]
            };

            let res = command.execute(bot, ctx, msg, &args).await;

            if (res.is_err()) {
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