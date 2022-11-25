use std::{fs::File, io::Write};

use chrono::{Datelike, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use serenity::{
    model::prelude::{ChannelId, GuildChannel, GuildId, Message, ReactionType},
    prelude::Context,
};

#[derive(Serialize, Deserialize, Debug)]
struct Friday {
    pub friday: bool,
    pub angels: Vec<u64>,
}

fn is_friday() -> bool {
    let offset = FixedOffset::east_opt(2 * 3600).unwrap();
    let now_utc = Utc::now();
    let now = now_utc.with_timezone(&offset);
    if now.weekday().to_string() == "Fri".to_string() {
        return true;
    }
    false
}

fn get_friday() -> Friday {
    let file = File::open("data/friday_data.json");
    let friday = match file {
        Ok(f) => {
            let friday: Friday = serde_json::from_reader(&f).expect("Unable to read file");
            friday
        }
        Err(_) => {
            let friday: Friday = Friday {
                friday: false,
                angels: Vec::new(),
            };
            let contents = serde_json::to_string_pretty(&friday).unwrap();
            File::create("data/friday_data.json")
                .unwrap()
                .write_all(contents.as_bytes())
                .unwrap();
            friday
        }
    };
    friday
}

async fn get_friday_channel(ctx: &Context) -> GuildChannel {
    let file = File::open("data/friday_channel.json");
    let (guild_id, channel_id) = match file {
        Ok(file) => {
            let content: (u64, u64) = serde_json::from_reader(file).expect("Unable to read file");
            content
        }
        Err(_) => {
            let channel: (u64, u64) = (0, 0);
            let contents = serde_json::to_string_pretty(&channel).unwrap();
            File::create("data/friday_channel.json")
                .unwrap()
                .write_all(contents.as_bytes())
                .unwrap();
            channel
        }
    };

    let guild_id = GuildId(guild_id);
    let guild = guild_id.to_partial_guild(&ctx.http).await.unwrap();
    let channel = guild
        .channels(&ctx.http)
        .await
        .unwrap()
        .get(&ChannelId(channel_id))
        .unwrap()
        .clone();
    channel
}

fn write_friday(friday: Friday) {
    let contents = serde_json::to_string_pretty(&friday).unwrap();
    File::create("data/friday_data.json")
        .unwrap()
        .write_all(contents.as_bytes())
        .unwrap();
}

pub fn add_user(user: u64) {
    let mut friday = get_friday();
    friday.angels.push(user);
    println!("{:?}", friday.angels);
    write_friday(friday);
}

pub fn remove_user(user: u64) {
    let mut friday = get_friday();
    friday.angels.retain(|&x| x != user);
    write_friday(friday);
}

pub async fn friday_controller(ctx: &Context) {
    let mut friday = get_friday();
    if friday.friday {
        if !is_friday() {
            friday.friday = false;
            write_friday(friday);
        }
    } else {
        if is_friday() {
            friday.friday = true;
            friday.angels = Vec::new();
            write_friday(friday);
            let channel = get_friday_channel(ctx).await;
            channel.say(&ctx.http, "Fiberoptic Friday!").await.unwrap();
            channel
                .say(&ctx.http, "https://tenor.com/view/fiber-optic-friday-eat-the-optic-cable-cable-shark-fibre-shark-gif-25548843")
                .await
                .unwrap();
        }
    }
}

fn is_sinner(user: u64) -> bool {
    let friday = get_friday();
    !friday.angels.contains(&user)
}

async fn friday_react(message: &Message, friday: bool, ctx: &Context) {
    let emote = if friday {
        ReactionType::try_from("<:Friday:1021853202964029470>").unwrap()
    } else {
        ReactionType::Unicode("🤓".to_string())
    };
    message.react(&ctx.http, emote).await.unwrap();
}

pub async fn handle_message(message: &Message, ctx: &Context) {
    let user = message.author.id.0;
    let friday = is_friday();
    let sinner = is_sinner(user);
    println!("{} {} {}", friday, sinner, user);
    if sinner && !message.author.bot {
        if friday
            && message.content.contains("tenor")
            && message.content.contains("fiber-optic-friday")
        {
            add_user(user);
            return;
        }
        friday_react(message, friday, ctx).await;
    }
}
