use sutombot::run_discord_bot;

use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("token");
    run_discord_bot(token.as_str()).await;
}
