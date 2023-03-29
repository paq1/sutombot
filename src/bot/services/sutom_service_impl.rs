use crate::core::services::sutom_service::SutomService;

use serenity::async_trait;
use serenity::futures::TryFutureExt;
use crate::core::entities::create_player::CreatePlayer;
use crate::core::entities::party::Party;
use crate::core::entities::player::Player;
use crate::models::views::player_score_view::PlayerScoreView;

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

    async fn classement(&self) -> Result<Vec<PlayerScoreView>, String> {
        let finaly = reqwest::get(format!("{}/players", self.url))
            .and_then(|response| {
                let res = response.json::<Vec<Player>>();
                res
            })
            .await
            .map(|players| {
                players
                    .into_iter()
                    .map(|player| {
                        PlayerScoreView {
                            score: Self::calcul_score(&player.parties),
                            name: player.name.to_string()
                        }
                    })
                    .collect::<Vec<_>>()
                    .sort_players()
            })
            .map_err(|_| "oops".to_string());

        finaly
    }
}

impl SutomServiceImpl {
    pub fn calcul_score(parties: &Vec<Party>) -> f32 {
        parties
            .into_iter()
            .map(|party| party.nombre_essaies)
            .sum::<u32>() as f32 / parties.len() as f32
    }
}

impl serenity::prelude::TypeMapKey for SutomServiceImpl {
    type Value = Self;
}

trait SortPlayersViewByScore {
    fn sort_players(&self) -> Vec<PlayerScoreView>;
}

impl SortPlayersViewByScore for Vec<PlayerScoreView> {
    fn sort_players(&self) -> Vec<PlayerScoreView> {
        let mut list = self.to_vec();
        list.sort_by(|a, b| a.score.total_cmp(&b.score));
        list
    }
}
