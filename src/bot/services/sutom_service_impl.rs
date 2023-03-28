use crate::core::services::sutom_service::SutomService;

use serenity::async_trait;
use serenity::futures::TryFutureExt;
use crate::core::entities::create_player::CreatePlayer;
use crate::core::entities::party::Party;
use crate::core::entities::player::Player;

#[derive(Clone)]
pub struct SutomServiceImpl {
    pub url: String
}

#[async_trait]
impl SutomService for SutomServiceImpl {
    async fn player_exist(&self, name: String) -> Result<bool, String> {
        reqwest::get(format!("{}/players", self.url))
            .and_then(|response| {
                let body = response.json::<Vec<Player>>();
                body
            })
            .await
            .map(|players| {
                players
                    .into_iter()
                    .any(|player| {
                        player.name == name
                    })
            })
            .map_err(|_| "une erreur est survenu".into())
    }

    async fn create_account(&self, name: String) -> Result<(), String> {
        reqwest::Client::new()
            .post(format!("{}/players/commands/create", self.url))
            .json(&CreatePlayer::new(name))
            .send()
            .await
            .map(|_| ())
            .map_err(|err| err.to_string())
    }

    async fn add_party(&self, party: Party, name: String) -> Result<u16, String> {
        reqwest::Client::new()
            .put(format!("{}/players/commands/add-party/{}", self.url, name))
            .json(&party)
            .send()
            .await
            .map(|response| response.status().as_u16())
            .map_err(|err| err.to_string())
    }
}

impl serenity::prelude::TypeMapKey for SutomServiceImpl {
    type Value = Self;
}
