use serenity::async_trait;
use crate::core::entities::party::Party;

#[async_trait]
pub trait SutomService {
    async fn player_exist(name: String) -> Result<bool, String>;
    async fn create_account(name: String) -> Result<(), String>;
    async fn add_party(party: Party, name: String) -> Result<u16, String>;
}