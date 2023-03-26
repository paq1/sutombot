use crate::core::services::sutom_service::SutomService;

use serenity::async_trait;
use serenity::futures::TryFutureExt;
use crate::core::entities::player::Player;

#[derive(Clone)]
pub struct SutomServiceImpl;

#[async_trait]
impl SutomService for SutomServiceImpl {
    async fn player_exist(&self, name: String) -> Result<bool, String> {
        reqwest::get("http://localhost:8000/players")
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
}

impl serenity::prelude::TypeMapKey for SutomServiceImpl {
    type Value = Self;
}