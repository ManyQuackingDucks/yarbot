use diesel::r2d2::ManageConnection;
use serenity::{
    client::Context,
    framework::standard::{macros::check, Reason},
    model::channel::Message,
};

use crate::SqliteClient;

pub mod general;
pub mod point;
pub mod raid;

#[check]
#[name = "bot_command"]
pub async fn bot_command(ctx: &Context, msg: &Message) -> Result<(), Reason> {
    channel_check(ctx, msg.channel_id.to_string(),"bot_channel_id").await
}

#[check]
#[name = "raid_command"]
pub async fn raid_command(ctx: &Context, msg: &Message) -> Result<(), Reason> {
    channel_check(ctx, msg.channel_id.to_string(),"raid_channel_id").await
}

#[check]
#[name = "point_command"]
pub async fn point_command(ctx: &Context, msg: &Message) -> Result<(), Reason> {
    channel_check(ctx, msg.channel_id.to_string(),"point_channel_id").await
}

async fn channel_check(ctx: &Context, channel_id: String, config_name: &str) -> Result<(), Reason>{
    let client = ctx.data.read().await;
    let sqlite = client.get::<SqliteClient>().unwrap();
    let mut conn = sqlite.connect().unwrap();
    let config_channel_id = match crate::config::get(config_name, &mut conn) {
        Ok(channel_id) => channel_id,
        Err(e) => {
            return Err(Reason::Log(e.to_string()));
        }
    };
    match config_channel_id {
        Some(config_channel_id) => {
            if channel_id == config_channel_id {
                return Ok(());
            }
            Err(Reason::User(
                "You can't use this command in this channel".to_string(),
            ))
        }
        None => {
            Err(Reason::User("Please set the bot channel".to_string()))
        }
    }
}