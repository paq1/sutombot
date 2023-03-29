use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::bot::services::sutom_service_impl::SutomServiceImpl;
use crate::core::services::sutom_service::SutomService;


pub async fn classement_command(ctx: &Context, msg: &Message) -> CommandResult {
    let sutom_service = &{
        let data = ctx.data.read().await;
        data.get::<SutomServiceImpl>().unwrap().clone()
    };

    let classement_global = sutom_service
        .classement()
        .await?;

    reply_standard(format!("{:?}", classement_global).as_str(), ctx, msg).await?;


    Ok(())
}

async fn reply_standard(content: &str, ctx: &Context, msg: &Message) -> Result<(), String> {
    msg
        .reply(ctx, content)
        .await
        .map(|_| ())
        .map_err(|err| err.to_string())
}