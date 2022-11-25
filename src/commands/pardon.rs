use std::fs::File;

use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType,
        interaction::application_command::{
            ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
        },
    },
};

use crate::commands::handler::Response;
use crate::friday::friday::add_user;

pub async fn run(
    options: &[CommandDataOption],
    command: &ApplicationCommandInteraction,
) -> Response {
    let user = command.member.as_ref().unwrap().user.id.0;
    let file = File::open("data/admin_users.json");
    if let Ok(file) = file {
        let admin_users: Vec<u64> = serde_json::from_reader(&file).expect("Unable to read file");
        if !admin_users.contains(&user) {
            return Response::Hidden("You are not an admin".to_string());
        }
    } else {
        return Response::Hidden("You are not an admin".to_string());
    }
    let user = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    if let CommandDataOptionValue::User(u, _) = user {
        add_user(u.id.0);
        return Response::Shown(format!("Added user {}", u.name));
    } else {
        return Response::Hidden("Please provide a valid name".to_string());
    };
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("user")
        .description("Creates a reaction")
        .create_option(|option| {
            option
                .name("name")
                .description("Choose a user to pardon")
                .kind(CommandOptionType::User)
                .required(true)
        })
}
