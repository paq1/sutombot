use serenity::async_trait;

#[async_trait]
pub trait SutomService {
    async fn player_exist(&self, name: String) -> Result<bool, String>;
}