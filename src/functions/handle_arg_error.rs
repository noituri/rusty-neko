use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};
use serenity::builder::{CreateEmbedAuthor, Timestamp};
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::utils::Color;
use crate::enums::raw_arg_types::raw_arg_types;
use crate::structures::arg::Arg;
use crate::structures::bot::Bot;

pub async fn handle_arg_error(bot: &Bot, ctx: &Context, msg: &Message, arg: &Arg, current: String, err: String) -> () {
    let iso = SystemTime::now();
    let iso: DateTime<Utc> = iso.into();
    let iso = iso.to_rfc3339();

    msg.channel_id.send_message(
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
                                raw_arg_types::Integer => {
                                    "Integer"
                                }

                                raw_arg_types::String => {
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

                    if !arg.example.is_empty() {
                        embed.field(
                            "Example",
                            arg.example.to_string(),
                            false
                        );
                    };

                    embed
                }
            )
        }
    ).await;
}