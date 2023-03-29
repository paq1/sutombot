use serenity::client::Context;
use serenity::model::channel::Message;

pub async fn reply_standard(content: &str, ctx: &Context, msg: &Message) -> Result<(), String> {
    msg
        .reply(ctx, content)
        .await
        .map(|_| ())
        .map_err(|err| err.to_string())
}