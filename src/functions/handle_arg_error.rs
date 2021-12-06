use std::time::{SystemTime};
use chrono::{DateTime, Utc};
use serenity::builder::{CreateEmbedAuthor};
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::utils::Color;
use crate::enums::raw_arg_types::RawArgTypes;
use crate::functions::get_command_usage::get_command_usage;
use crate::functions::plural_for::plural_for;
use crate::structures::arg::Arg;
use crate::structures::bot::Bot;
use crate::structures::extras::Extras;
use crate::traits::command_trait::Command;

pub async fn handle_arg_error(_bot: &Bot, ctx: &Context, command: &Box<dyn Command>, extras: &Extras, msg: &Message, arg: &Arg, current: String, err: String) -> () {
    let iso = SystemTime::now();
    let iso: DateTime<Utc> = iso.into();
    let iso = iso.to_rfc3339();

    let usage = get_command_usage(command, extras);

    let _ = msg.channel_id.send_message(
        ctx,
        | m | {
            m.add_embed(
                | embed | {
                    let mut auth = CreateEmbedAuthor::default();
                    auth.name(msg.author.tag())
                        .icon_url(msg.author.avatar_url().unwrap());

                    embed.color(Color::RED)
                        .title("Argument Error")
                        .set_author(auth)
                        .description(
                            {
                                if current.is_empty() {
                                    "No input was provided for a required argument.".to_string()
                                } else {
                                    err
                                }
                            }
                        )
                        .footer(| ft | {
                            ft.text("Limitations...")
                        })
                        .timestamp(iso)
                        .field("Expected", {
                            match arg.expect {
                                RawArgTypes::Integer => {
                                    "Integer"
                                }

                                RawArgTypes::Bool => {
                                    "Boolean"
                                }

                                RawArgTypes::String => {
                                    "String"
                                }
                            }
                        }, false)
                        .field("Got", {
                            if current.is_empty() {
                                "None.".to_string()
                            } else if current.len() > 1024{
                                current[0..1024].to_string()
                            } else {
                                current
                            }
                        }, false);

                    if arg.min_len != 0 || arg.max_len != 0 {
                        embed.field(
                            "Range",
                            format!(
                                "{}-{}",
                                arg.min_len,
                                {
                                    if arg.max_len != 0 {
                                        arg.max_len.to_string()
                                    } else {
                                        "?".to_string()
                                    }
                                }
                            ),
                            true
                        );
                    };

                    if !arg.example.is_empty() {
                        embed.field(
                            "Argument Example",
                            arg.example.to_string(),
                            false
                        );
                    };

                    if !usage.is_empty() {
                        embed.field(
                            plural_for("Command Usage", usage.len()),
                            format!(
                                "```\n{}```",
                                usage.join("\n")
                            ),
                            false
                        );
                    };

                    embed
                }
            )
        }
    ).await;
}