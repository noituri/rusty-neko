use serenity::client::Context;
use serenity::model::interactions::{Interaction, InteractionType};
use serenity::model::interactions::message_component::ComponentType;
use crate::handlers::interactions::handle_dismiss_interaction::handle_dismiss_interaction;
use crate::structures::bot::Bot;

pub async fn interaction_create(bot: &Bot, ctx: &Context, interaction: Interaction) {
    if interaction.kind() == InteractionType::MessageComponent {
        let component = interaction.message_component().unwrap();

        #[allow(clippy::single_match)]
        match component.data.component_type {
            ComponentType::Button => {
                handle_dismiss_interaction(bot, ctx, component).await;
            }

            _ => ()
        }
    }
}