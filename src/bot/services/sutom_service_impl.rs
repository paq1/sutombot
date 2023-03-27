use crate::core::services::sutom_service::SutomService;

use serenity::async_trait;
use serenity::futures::TryFutureExt;
use crate::core::entities::create_player::CreatePlayer;
use crate::core::entities::party::Party;
use crate::core::entities::player::Player;

#[derive(Clone)]
pub struct SutomServiceImpl;

#[async_trait]
impl SutomService for SutomServiceImpl {
    async fn player_exist(name: String) -> Result<bool, String> {
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

    async fn create_account(name: String) -> Result<(), String> {
        reqwest::Client::new()
            .post("http://localhost:8000/players/commands/create")
            .json(&CreatePlayer::new(name))
            .send()
            .await
            .map(|_| ())
            .map_err(|err| err.to_string())
    }

    async fn add_party(party: Party, name: String) -> Result<(), String> {
        reqwest::Client::new()
            .put(format!("http://localhost:8000/players/commands/add-party/{name}"))
            .json(&party)
            .send()
            .await
            .map(|_| ())
            .map_err(|err| err.to_string())
    }
}
