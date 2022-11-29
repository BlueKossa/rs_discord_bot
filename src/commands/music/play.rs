use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType,
        interaction::application_command::{
            ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
        },
    },
    prelude::Context,
};

use crate::commands::handler::Response;

pub async fn run(
    options: &[CommandDataOption],
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Response {
    let link = options.get(0).unwrap().resolved.as_ref().unwrap();

    if let CommandDataOptionValue::String(link) = link {}

    Response::Hidden("Temp".to_string())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("play")
        .description("Play a song")
        .create_option(|option| {
            option
                .name("link")
                .description("link to a song or playlist")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
