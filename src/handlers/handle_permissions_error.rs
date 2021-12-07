use std::convert::TryInto;
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use serenity::builder::CreateEmbedAuthor;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::interactions::message_component::ButtonStyle;
use serenity::utils::Color;
use crate::enums::permission_errors::PermissionErrors;
use crate::structures::bot::Bot;
use crate::structures::extras::Extras;
use crate::traits::command_trait::Command;

pub async fn handle_permissions_error(bot: &Bot, msg: &Message, cmd: &Box<dyn Command>, ctx: &Context, extras: &Extras, perm_error: PermissionErrors) -> () {
    let _ = msg.channel_id.send_message(
        ctx,
        | m | {
            m.components(| c | {
                c.create_action_row(| row | {
                    row.create_button(| btn | {
                        btn.label("Dismiss")
                            .custom_id(
                                format!(
                                    "{}_{}",
                                    "dismiss",
                                    msg.author.id
                                )
                            )
                            .style(ButtonStyle::Danger)
                    })
                })
            });

            m.embed(
                | embed | {
                    let iso = SystemTime::now();
                    let iso: DateTime<Utc> = iso.into();
                    let iso = iso.to_rfc3339();

                    embed.author(| auth | {
                        auth.name(msg.author.tag())
                            .icon_url(msg.author.avatar_url().unwrap())
                    })
                        .timestamp(iso)
                        .footer(| ft | {
                            ft.text("Limitations...")
                        })
                        .color(Color::RED)
                        .description({
                            match perm_error {
                                PermissionErrors::STAFF => "Only staff members can execute this command.",
                                PermissionErrors::OWNER => "Only owners can execute this command."
                            }
                        })
                }
            )
        }
    ).await;
}