use std::fs::File;
use std::io::Write;

use serde::{Deserialize, Serialize};
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::autocomplete::AutocompleteInteraction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::ReactionType;
use serenity::prelude::Context;

use crate::commands::handler::Response;

#[derive(Deserialize, Serialize, Debug)]
pub struct Emote {
    pub name: String,
    pub emote: String,
}

struct React {
    emote: ReactionType,
    id: Option<u64>,
    relative: u64,
}

impl React {
    fn new() -> Self {
        Self {
            emote: ReactionType::Unicode(String::new()),
            id: None,
            relative: 1,
        }
    }
}

pub async fn run(
    options: &[CommandDataOption],
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Response {
    let mut react = React::new();
    for option in options {
        match option.name.as_str() {
            "emote" => {
                if let CommandDataOptionValue::String(emote) = &option.resolved.as_ref().unwrap() {
                    let file = File::open("data/emotes.json").expect("Unable to open file");
                    let emotes: Vec<Emote> =
                        serde_json::from_reader(&file).expect("Unable to read file");
                    let find_emote = emotes
                        .iter()
                        .find(|e| e.name == emote.to_string() || e.emote == emote.to_string());
                    if let Some(emote) = find_emote {
                        let emote_string = format!("<:{}:{}>", emote.name, emote.emote);
                        if let Ok(e) = ReactionType::try_from(emote_string) {
                            react.emote = e;
                        } else {
                            return Response::Hidden("Unable to find emote".to_string());
                        }
                    } else {
                        return Response::Hidden("Unable to find emote".to_string());
                    }
                }
            }
            "id" => {
                if let CommandDataOptionValue::String(id) = &option.resolved.as_ref().unwrap() {
                    let id = id.parse::<u64>();
                    if let Ok(id) = id {
                        react.id = Some(id);
                    } else {
                        return Response::Hidden("Invalid message ID".to_string());
                    }
                }
            }
            "relative" => {
                if let CommandDataOptionValue::Integer(relative) =
                    &option.resolved.as_ref().unwrap()
                {
                    react.relative = *relative as u64;
                }
            }
            _ => {}
        }
    }
    let channel = command
        .channel_id
        .to_channel(&ctx.http)
        .await
        .unwrap()
        .guild()
        .unwrap();
    let message;
    if let Some(id) = react.id {
        message = channel.message(&ctx.http, id).await.unwrap();
    } else {
        let messages = channel
            .messages(&ctx.http, |m| m.limit(react.relative))
            .await
            .unwrap();
        message = messages.first().unwrap().to_owned();
    }
    message
        .react(&ctx.http, react.emote.to_owned())
        .await
        .unwrap();
    Response::Hidden("Successfully reacted".to_string())
    /*     let option = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");
    println!("{:?}", option);
    let emote;
    if let CommandDataOptionValue::String(str) = option {
        println!("{}", str);
        let file = File::open("data/emotes.json").expect("Unable to open file");
        let emotes: Vec<Emote> = serde_json::from_reader(&file).expect("Unable to read file");
        let find_emote = emotes
            .iter()
            .find(|e| e.name == str.to_string() || e.emote == str.to_string());
        match find_emote {
            Some(e) => {
                let emote_string = format!("<:{}:{}>", e.name, e.emote);
                if let Ok(e) = ReactionType::try_from(emote_string) {
                    emote = e;
                } else {
                    return Response::Hidden("Invalid emote".to_string());
                }
            }
            None => return Response::Hidden("No emote by that name exists".to_string()),
        }
    } else {
        return Response::Hidden("Failed to react".to_string());
    }

    let option = options
        .get(1)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");
    println!("{:?}", options.len());
    if let CommandDataOptionValue::Integer(int) = option {
        let channel = command
            .channel_id
            .to_channel(&ctx.http)
            .await
            .unwrap()
            .guild()
            .unwrap();

        if int <= &20 {
            let messages = channel
                .messages(&ctx.http, |m| m.limit(*int as u64))
                .await
                .unwrap();
            let message = messages.last().unwrap();
            message.react(&ctx.http, emote).await.unwrap();
        } else {
            let message = channel.message(&ctx.http, *int as u64).await.unwrap();
            message.react(&ctx.http, emote).await.unwrap();
        }

        return Response::Hidden("Successfully reacted".to_string());
    } else {
        return Response::Hidden("Failed to react".to_string());
    } */
}

pub async fn send_autocomplete(autocomplete: &AutocompleteInteraction, ctx: &Context) {
    // Open the emote file and read all emotes into a vector
    let file = File::open("data/emotes.json").expect("Unable to open file");
    let emotes: Vec<Emote> = serde_json::from_reader(file).expect("Unable to read file");
    // Return a response with all the emotes
    let autocomplete = autocomplete.create_autocomplete_response(&ctx.http, |response| {
        for emote in emotes {
            response.add_string_choice(emote.name, emote.emote);
        }
        response
    });

    autocomplete.await.expect("Unable to send autocomplete");
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    // Create the required files if they don't exist
    let file = File::open("data/emotes.json");
    match file {
        Ok(_) => {}
        Err(_) => {
            let emotes: Vec<Emote> = Vec::new();
            let contents = serde_json::to_string_pretty(&emotes).unwrap();
            File::create("data/emotes.json")
                .unwrap()
                .write_all(contents.as_bytes())
                .unwrap();
        }
    };
    command
        .name("react")
        .description("Get a user id")
        .create_option(|option| {
            option
                .name("emote")
                .description("Choose an emote to react with")
                .kind(CommandOptionType::String)
                .required(true)
                .set_autocomplete(true)
        })
        .create_option(|option| {
            option
                .name("id")
                .description("React to a message by id")
                .kind(CommandOptionType::String)
                .required(false)
        })
        .create_option(|option| {
            option
                .name("relative")
                .description("X messages ago to react to, defaults to last message in channel")
                .kind(CommandOptionType::Number)
                .required(false)
                .max_int_value(20)
                .min_int_value(1)
        })
}
