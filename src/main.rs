mod commands;

use std::env;

use commands::handler::{autocomplete_handler, command_handler};
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match &interaction {
            Interaction::ApplicationCommand(command) => {
                command_handler(&ctx, &command).await;
            }
            Interaction::Autocomplete(autocomplete) => {
                autocomplete_handler(&ctx, &autocomplete).await;
            }
            _ => {
                println!("Received interaction: {:#?}, not implemented!", interaction);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_command = Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::react::register(command))
                .create_application_command(|command| commands::create_reaction::register(command))
        })
        .await;

        println!(
            "I created the following global slash command: {:#?}",
            guild_command
        );
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::args().nth(1).expect("Expected a bot token");

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
