use serenity::framework::standard::CommandResult;
use serenity::futures::TryFutureExt;
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use sutom_rules::core::message_handler::handle_message;
use crate::bot::services::message_service::reply_standard;

use crate::bot::services::sutom_service_impl::SutomServiceImpl;
use crate::core::entities::party::Party;
use crate::core::services::sutom_service::SutomService;


pub async fn partie_command(ctx: &Context, msg: &Message) -> CommandResult {

    let sutom_service = &{
        let data = ctx.data.read().await;
        data.get::<SutomServiceImpl>().unwrap().clone()
    };

    let content: String = msg.content.clone();
    let party = handle_message(&content)
        .map(|party| Party::new(party.taille_du_mot, party.nombre_essaies_total, party.nombre_essaies))?;

    let user = msg.author.name.clone();

    sutom_service.player_exist(user.clone())
        .and_then(|response| async move {
            if response.clone() {
                sutom_service.add_party(party.clone(), user.clone())
                    .and_then(|res| async move {
                        if res >= 400 {
                            reply_standard("vous avez déjà joué aujourd'hui 😋", ctx, msg).await
                        } else {
                            reply_standard("la partie a bien ete ajoutée 😘", ctx, msg).await
                        }
                    })
                    .await
            } else {
                sutom_service.create_account(user.clone())
                    .and_then(|_| async {
                        reply_standard("votre compte a bien été créé 🤖", ctx, msg).await
                    })
                    .and_then(|_| async move {
                        reply_standard("la partie a bien ete ajoutée 😘", ctx, msg)
                            .map_err(|err| err.to_string())
                            .and_then(|_| async move {
                                sutom_service.add_party(party.clone(), user.clone())
                                    .await
                                    .map(|_| ())
                            })
                            .await
                    })
                    .await
            }
        })
        .await?;

    Ok(())
}
