use std::fs::File;
use std::io::Write;

use serde::{Deserialize, Serialize};
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::autocomplete::AutocompleteInteraction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::prelude::Context;

#[derive(Deserialize, Serialize, Debug)]
pub struct Emote {
    pub name: String,
    pub emote: String,
}

pub fn run(options: &[CommandDataOption]) -> String {
    let option = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");
    println!("{:?}", option);

    if let CommandDataOptionValue::String(str) = option {
        format!("command input is {}", str)
    } else {
        "Please provide a valid user".to_string()
    }
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
                .name("emotes")
                .description("Choose an emote")
                .kind(CommandOptionType::String)
                .required(false)
                .set_autocomplete(true)
        })
        .create_option(|option| {
            option
                .name("message")
                .description("X messages ago to react to, defaults to last message in channel")
                .kind(CommandOptionType::Integer)
                .required(false)
                .max_int_value(20)
        })
}
