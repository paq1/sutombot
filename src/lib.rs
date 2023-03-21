mod services;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};
use crate::services::score_service::ScoreService;

#[group]
#[commands(ping, score)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

struct Counter;
impl TypeMapKey for Counter {
    type Value = u32;
}

pub async fn run_discord_bot(token: &str) {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<Counter>(1);
    }

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let counter = {
        let data = ctx.data.read().await;
        data.get::<Counter>().unwrap().clone()
    };
    let new_counter = counter + 1;
    msg
        .reply(ctx, format!("conteur : {}", new_counter)).await?;

    {
        let mut data = ctx.data.write().await;
        data.insert::<Counter>(new_counter);
    }

    Ok(())
}

#[command]
async fn score(ctx: &Context, msg: &Message) -> CommandResult {

    let content: String = msg.content.clone();
    let traitement: String = ScoreService::handle_message(&content).unwrap();

    msg
        .reply(ctx, format!("{}", traitement)).await?;

    Ok(())
}