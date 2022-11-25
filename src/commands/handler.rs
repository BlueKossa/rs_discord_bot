use serenity::{
    model::prelude::interaction::{
        application_command::ApplicationCommandInteraction, autocomplete::AutocompleteInteraction,
        InteractionResponseType,
    },
    prelude::Context,
};

use crate::commands;

pub enum Response {
    Shown(String),
    Hidden(String),
}

pub async fn command_handler(ctx: &Context, command: &ApplicationCommandInteraction) {
    let res = match command.data.name.as_str() {
        "react" => commands::react::run(&command.data.options, &ctx, &command).await,
        "createreaction" => {
            commands::create_reaction::run(&command.data.options, &ctx, &command).await
        }
        "pardon" => commands::pardon::run(&command.data.options, &command).await,
        _ => Response::Hidden("Unknown command".to_string()),
    };
    response_handler(&ctx, &command, &res).await;
}

async fn response_handler(ctx: &Context, command: &ApplicationCommandInteraction, res: &Response) {
    if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| match res {
                    Response::Shown(content) => message.content(content),
                    Response::Hidden(content) => message.content(content).ephemeral(true),
                })
        })
        .await
    {
        println!("Cannot respond to slash command: {}", why);
    }
}

pub async fn autocomplete_handler(ctx: &Context, autocomplete: &AutocompleteInteraction) {
    println!("Hello");
    match autocomplete.data.name.as_str() {
        "react" => commands::react::send_autocomplete(autocomplete, &ctx).await,
        _ => {}
    };
}
