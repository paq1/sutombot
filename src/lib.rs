use std::env;
use std::fmt::{Debug, Display, Formatter};

use serenity::async_trait;
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::framework::standard::macros::{command, group};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::bot::commands_discord::classement_command::classement_command;
use crate::bot::commands_discord::partie_command::partie_command;
use crate::bot::services::party_service_impl::PartyServiceImpl;
use crate::bot::services::sutom_service_impl::SutomServiceImpl;

mod core;
mod bot;
mod models;

#[group]
#[commands(partie, classement)]
struct General;

#[command]
async fn partie(ctx: &Context, msg: &Message) -> CommandResult {
    partie_command(ctx, msg).await
}

#[command]
async fn classement(ctx: &Context, msg: &Message) -> CommandResult {
    classement_command(ctx, msg).await
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

struct Counter;
impl TypeMapKey for Counter {
    type Value = u32;
}

#[derive(Debug)]
struct MonErreur;

impl Display for MonErreur {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mon Erreur")
    }
}

impl std::error::Error for MonErreur {

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
        data.insert::<PartyServiceImpl>(PartyServiceImpl {});
        data.insert::<SutomServiceImpl>(SutomServiceImpl { url: env::var("SUTOM_API_URL").expect("url manquante") })
    }

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
