use diesel::r2d2::ManageConnection;
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
};

use crate::SqliteClient;

#[group]
#[commands(set)]
struct General;

#[command]
#[only_in(guilds)]
#[allowed_roles("Captain")]
async fn set(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let channel_id = msg.channel_id.to_string();
    let data = ctx.data.read().await;
    let conn_manager = data
        .get::<SqliteClient>()
        .ok_or("Could not get connection manager")?;
    let mut conn = conn_manager.connect()?;
    crate::config::set("bot_channel_id", &channel_id, &mut conn)?;
    msg.reply(&ctx.http, "Set bot command channel").await?;
    Ok(())
}
