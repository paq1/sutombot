use std::env;
use std::fmt::{Debug, Display, Formatter};

use serenity::async_trait;
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::framework::standard::macros::{command, group};
use serenity::futures::TryFutureExt;
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::bot::services::party_service_impl::PartyServiceImpl;
use crate::bot::services::sutom_service_impl::SutomServiceImpl;
use crate::core::services::party_service::PartyService;
use crate::core::services::sutom_service::SutomService;

mod core;
mod bot;

#[group]
#[commands(ping, partie)]
struct General;

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
async fn partie(ctx: &Context, msg: &Message) -> CommandResult {
    let party_service = {
        let data = ctx.data.read().await;
        data.get::<PartyServiceImpl>().unwrap().clone()
    };

    let sutom_service = &{
        let data = ctx.data.read().await;
        data.get::<SutomServiceImpl>().unwrap().clone()
    };

    let content: String = msg.content.clone();
    let party = party_service.handle_message(&content)?;

    let user = msg.author.name.clone();

    sutom_service.player_exist(user.clone())
        .and_then(|response| async move {
            if response.clone() {
                sutom_service.add_party(party.clone(), user.clone())
                    .and_then(|res| async move {
                        if res >= 400 {
                            reply_standard("vous avez déjà joué aujourd'hui 😋", ctx, msg).await
                        } else {
                            reply_standard("la partie a bien ete ajoutée 😘", ctx, msg).await
                        }
                    })
                    .await
            } else {
                sutom_service.create_account(user.clone())
                    .and_then(|_| async {
                        reply_standard("votre compte a bien été créé 🤖", ctx, msg).await
                    })
                    .and_then(|_| async move {
                        reply_standard("la partie a bien ete ajoutée 😘", ctx, msg)
                            .map_err(|err| err.to_string())
                            .and_then(|_| async move {
                                sutom_service.add_party(party.clone(), user.clone())
                                    .await
                                    .map(|_| ())
                            })
                            .await
                    })
                    .await
            }
        })
        .await
        .map_err(|_| MonErreur {})
        .expect("erreur");

    Ok(())
}

async fn reply_standard(content: &str, ctx: &Context, msg: &Message) -> Result<(), String> {
    msg
        .reply(ctx, content)
        .await
        .map(|_| ())
        .map_err(|err| err.to_string())
}