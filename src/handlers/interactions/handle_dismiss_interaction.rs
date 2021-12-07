use serenity::client::Context;
use serenity::model::interactions::InteractionResponseType;
use serenity::model::prelude::InteractionApplicationCommandCallbackDataFlags;
use serenity::model::prelude::message_component::MessageComponentInteraction;
use crate::structures::bot::Bot;

pub async fn handle_dismiss_interaction(_bot: &Bot, ctx: &Context, interaction: MessageComponentInteraction) {
    if !interaction.data.custom_id.contains("dismiss") {
        return;
    }

    if !interaction.data.custom_id.contains(&interaction.user.id.to_string()) {
        let _ = interaction.create_interaction_response(
            ctx,
            | i | {
                i.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(| r | {
                        r.content("You cannot use this button.")
                            .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                    })
            }
        ).await;
        return;
    }

    let _ = interaction
        .message
        .regular()
        .unwrap()
        .delete(
            ctx
        )
        .await;
}