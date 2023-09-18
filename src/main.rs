mod commands;
mod friday;

use std::env;

use commands::handler::{autocomplete_handler, command_handler};
use friday::friday::handle_message;
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::prelude::Message;
use serenity::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use rustrict::CensorStr;

use crate::friday::friday::friday_controller;

static THREAD: AtomicBool = AtomicBool::new(false);

const GIF: &'static str = "https://media.discordapp.net/attachments/1153318707221762090/1153346066754838610/ezgif-2-21e2311d2b.gif";

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

    async fn message(&self, ctx: Context, msg: Message) {
        println!("Received message: {:#?}", msg);
        if msg.content.is_inappropriate() {
            msg.reply(&ctx, GIF).await.unwrap();
        }
        if std::env::args().any(|arg| arg == "--DFriday".to_owned()) {
            return;
        }
        handle_message(&msg, &ctx).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let guild_command = Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::react::register(command))
                .create_application_command(|command| commands::create_reaction::register(command))
                .create_application_command(|command| commands::pardon::register(command))
        })
        .await;
        if std::env::args().any(|arg| arg == "--DFriday".to_string()) {
            return;
        }
        if THREAD.load(Ordering::Relaxed) {
            return;
        }
        THREAD.store(true, Ordering::Relaxed);
        tokio::spawn(async move {
            loop {
                friday_controller(&ctx).await;
                tokio::time::sleep(std::time::Duration::from_secs(60)).await;
            }
        });
        println!("Registered commands: {:#?}", guild_command);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::args().nth(1).expect("Expected a bot token");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    // Build our client.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
