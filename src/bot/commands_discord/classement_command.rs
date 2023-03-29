use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::bot::services::sutom_service_impl::SutomServiceImpl;
use crate::core::services::sutom_service::SutomService;
use crate::bot::services::message_service::reply_standard;
use crate::models::views::player_score_view::PlayerScoreView;

pub async fn classement_command(ctx: &Context, msg: &Message) -> CommandResult {
    let sutom_service = &{
        let data = ctx.data.read().await;
        data.get::<SutomServiceImpl>().unwrap().clone()
    };

    let classement_global = sutom_service
        .classement()
        .await?;

    let affichage = formatted_classement(&classement_global);

    reply_standard(affichage.as_str(), ctx, msg).await?;


    Ok(())
}

fn formatted_classement(players: &Vec<PlayerScoreView>) -> String {
    let classement = players
        .into_iter()
        .map(|player| {
            format!("joueur : {} avec le score de {}", player.name, player.score)
        })
        .collect::<Vec<_>>();

    let empty_classement = (0..3)
        .map(|_| String::from("???"))
        .collect::<Vec<_>>();

    classement
        .into_iter()
        .chain(empty_classement.into_iter())
        .collect::<Vec<_>>()
        .into_iter()
        .enumerate()
        .map(|element| {
            if element.0 == 0 {
                format!("🤑 - {}", element.1)
            } else if element.0 == 1 {
                format!("😎 - {}", element.1)
            } else {
                format!("😭 - {}", element.1)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

