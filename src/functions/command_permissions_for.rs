use std::error::Error;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::id::RoleId;
use crate::config::OWNERS;
use crate::enums::permission_errors::PermissionErrors;
use crate::functions::get_staff_role_id::get_staff_role_id;
use crate::handlers::handle_permissions_error::handle_permissions_error;
use crate::structures::bot::Bot;
use crate::structures::extras::Extras;
use crate::traits::command_trait::Command;
use crate::util::constants::staff_roles::get_staff_roles;

#[allow(box_pointers)]
pub async fn command_permissions_for(bot: &Bot, cmd: &Box<dyn Command>, ctx: &Context, msg: &Message, extras: &Extras, sendm: bool) -> Result<bool, Box<dyn Error + Sync + Send>> {
    if cmd.owner_only() && !OWNERS.iter().any(| m | *m == msg.author.id.to_string()) {
        if sendm {
            handle_permissions_error(bot, msg, cmd, ctx, extras, PermissionErrors::Owner).await;
        }
        return Ok(false);
    };

    if cmd.staff_only() {
        let member = msg.member.as_ref().unwrap();

        if !get_staff_roles().iter().any(| (_, id) | member.roles.contains(&RoleId(*id))) {
            if sendm {
                handle_permissions_error(bot, msg, cmd, ctx, extras, PermissionErrors::Staff).await;
            }

            return Ok(false);
        }
    }

    if !cmd.staff_roles().is_empty() {
        let member = msg.member.as_ref().unwrap();

        if !cmd.staff_roles().iter().any(| f | member.roles.contains(&RoleId(get_staff_role_id(f)))) {
            if sendm {
                handle_permissions_error(bot, msg, cmd, ctx, extras, PermissionErrors::Staff).await;
            }

            return Ok(false);
        }
    }

    Ok(true)
}