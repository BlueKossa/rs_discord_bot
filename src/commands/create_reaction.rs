use std::fs::{File, OpenOptions};
use std::io::Write;

extern crate base64;

use serde::{Deserialize, Serialize};
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::GuildId;
use serenity::prelude::Context;

use crate::commands::handler::Response;
use crate::commands::react::Emote;

#[derive(Deserialize, Serialize, Debug)]
struct HomeChannel {
    guild_id: u64,
    channel_id: u64,
}

pub async fn run(options: &[CommandDataOption], ctx: &Context) -> Response {
    // Open the emote file and read all emotes into a mutable vector
    let file = File::open("data/emotes.json").expect("Unable to open file");
    let mut emotes: Vec<Emote> = serde_json::from_reader(&file).expect("Unable to read file");
    // Get the home channel from a file
    let file = File::open("data/home_channel.json");
    let home_channel = match file {
        Ok(_) => {
            let file = File::open("data/home_channel.json").expect("Unable to open file");
            let home_channel: HomeChannel =
                serde_json::from_reader(file).expect("Unable to read file");
            home_channel
        }
        Err(_) => HomeChannel {
            guild_id: 0,
            channel_id: 0,
        },
    };
    // Get the guild of the home channel
    let guild_id = GuildId(home_channel.guild_id);
    let guild = guild_id.to_partial_guild(&ctx.http).await.unwrap();

    // Get the name of the emote
    let option = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");
    let name: String;
    if let CommandDataOptionValue::String(str) = option {
        for emote in &emotes {
            if emote.name == *str {
                return Response::Hidden("An emote by that name already exists".to_string());
            }
        }
        name = str.to_string();
    } else {
        return Response::Hidden("Please provide a valid name".to_string());
    };

    // Get the emote attachment
    let option = options
        .get(1)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    if let CommandDataOptionValue::Attachment(attachment) = option {
        // Download the image into a buffer
        let bytes = attachment.download().await.unwrap();
        // Encode the buffer into a base64 string
        let b64 = base64::encode(bytes);
        // Get the file extension, default to jpg
        let ext = match attachment.filename.split('.').last() {
            Some(ext) if ext == "png" => ext,
            _ => "jpg",
        };
        // Format the string into a data url
        let formated = format!("data:image/{};base64,{}", ext, b64);
        // Create a new emote
        let emoji = match guild
            .create_emoji(&ctx.http, name.as_str(), &formated)
            .await
        {
            Ok(emoji) => emoji,
            Err(_) => return Response::Hidden("Please provide a valid image".to_string()),
        };
        // Add the emote to the emote vector
        emotes.push(Emote {
            name: emoji.name,
            emote: emoji.id.to_string(),
        });
        // Open the emote file and write the emote vector to it
        let contents = serde_json::to_string_pretty(&emotes).unwrap();
        OpenOptions::new()
            .write(true)
            .open("data/emotes.json")
            .unwrap()
            .write_all(contents.as_bytes())
            .unwrap();
        return Response::Shown(format!("Added new emote '{}'", name));
    } else {
        return Response::Hidden("Please provide a valid image".to_string());
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    // Create the required files if they don't exist
    let file = File::open("data/home_channel.json");
    match file {
        Ok(_) => {}
        Err(_) => {
            File::create("data/home_channel.json").unwrap();
            let emotes: HomeChannel = HomeChannel {
                guild_id: 0,
                channel_id: 0,
            };
            let contents = serde_json::to_string_pretty(&emotes).unwrap();
            File::create("data/home_channel.json")
                .unwrap()
                .write_all(contents.as_bytes())
                .unwrap();
        }
    };
    command
        .name("createreaction")
        .description("Creates a reaction")
        .create_option(|option| {
            option
                .name("name")
                .description("Choose a name for the emote")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("media")
                .description("Choose an image or gif for the emote")
                .kind(CommandOptionType::Attachment)
                .required(true)
        })
}
