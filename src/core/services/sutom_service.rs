use serenity::async_trait;
use crate::core::entities::party::Party;

#[async_trait]
pub trait SutomService {
    async fn player_exist(&self, name: String) -> Result<bool, String>;
    async fn create_account(&self, name: String) -> Result<(), String>;
    async fn add_party(&self, party: Party, name: String) -> Result<u16, String>;
}