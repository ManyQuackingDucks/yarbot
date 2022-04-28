use serenity::client::Context;
use serenity::framework::standard::macros::hook;
use serenity::framework::standard::{CommandError, DispatchError, Reason};
use serenity::model::channel::Message;
#[hook]
pub async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError, _: &str) {
    log::info!("Dispatch Error occured");
    #[allow(clippy::single_match_else)] //Will add more later
    match error {
        DispatchError::LackingRole => {
            msg.reply(&ctx, "You don't have the required role.")
                .await
                .unwrap();
        }
        DispatchError::CheckFailed(_, e) => match e {
            Reason::User(e) => {
                msg.reply(&ctx, e.to_string()).await.unwrap();
            }
            Reason::UserAndLog { user, log } => {
                msg.reply(&ctx, user).await.unwrap();
                log::error!("{log}");
            }
            Reason::Log(e) => {
                log::error!("{e}");
                msg.reply(&ctx, "Sorry an error occured").await.unwrap();
            }
            Reason::Unknown => {
                msg.reply(&ctx, "Sorry an error occured").await.unwrap();
            }
            _ => {
                msg.reply(&ctx, "Sorry an error occured").await.unwrap();
            }
        },
        _ => {
            msg.reply(&ctx, "Sorry, an error occured.").await.unwrap();
            log::error!("Dispatch error: {:?}", error);
        }
    }
}

#[hook]
pub async fn after_hook(
    ctx: &Context,
    msg: &Message,
    cmd_name: &str,
    error: Result<(), CommandError>,
) {
    //  Print out an error if it happened
    if let Err(why) = error {
        msg.reply(&ctx.http, "Sorry an error occured.")
            .await
            .unwrap();
        log::error!("{cmd_name} - {why:?}");
    }
}
