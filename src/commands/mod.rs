use diesel::r2d2::ManageConnection;
use serenity::{
    client::Context,
    framework::standard::{macros::check, Reason},
    model::channel::Message,
};

use crate::SqliteClient;

pub mod point;
pub mod raid;
pub mod general;

#[check]
#[name = "bot_command"]
pub async fn bot_command(ctx: &Context, msg: &Message) -> Result<(), Reason> {
    let client = ctx.data.read().await;
    let sqlite = client.get::<SqliteClient>().unwrap();
    let mut conn = sqlite.connect().unwrap();
    let channel_id = msg.channel_id.0;
    let config_channel_id = match crate::config::get("bot_channel_id", &mut conn) {
        Ok(channel_id) => channel_id,
        Err(e) => {
            return Err(Reason::Log(e.to_string()));
        }
    };
    match config_channel_id {
        Some(config_channel_id) => {
            if channel_id.to_string() == config_channel_id {
                return Ok(());
            } else {
                return Err(Reason::User(
                    "You can't use this command in this channel".to_string(),
                ));
            }
        }
        None => {
            return Err(Reason::User("Please set the bot channel".to_string()));
        }
    }
}

#[check]
#[name = "raid_command"]
pub async fn raid_command(ctx: &Context, msg: &Message) -> Result<(), Reason> {
    let client = ctx.data.read().await;
    let sqlite = client.get::<SqliteClient>().unwrap();
    let mut conn = sqlite.connect().unwrap();
    let channel_id = msg.channel_id.0;
    let config_channel_id = match crate::config::get("raid_channel_id", &mut conn) {
        Ok(channel_id) => channel_id,
        Err(e) => {
            return Err(Reason::Log(e.to_string()));
        }
    };
    match config_channel_id {
        Some(config_channel_id) => {
            if channel_id.to_string() == config_channel_id {
                return Ok(());
            } else {
                return Err(Reason::User(
                    "You can't use this command in this channel".to_string(),
                ));
            }
        }
        None => {
            return Err(Reason::User("Please set the raid channel".to_string()));
        }
    }
}

#[check]
#[name = "point_command"]
pub async fn point_command(ctx: &Context, msg: &Message) -> Result<(), Reason> {
    let client = ctx.data.read().await;
    let sqlite = client.get::<SqliteClient>().unwrap();
    let mut conn = sqlite.connect().unwrap();
    let channel_id = msg.channel_id.0;
    let config_channel_id = match crate::config::get("point_channel_id", &mut conn) {
        Ok(channel_id) => channel_id,
        Err(e) => {
            return Err(Reason::Log(e.to_string()));
        }
    };
    match config_channel_id {
        Some(config_channel_id) => {
            if channel_id.to_string() == config_channel_id {
                return Ok(());
            } else {
                return Err(Reason::User(
                    "You can't use this command in this channel".to_string(),
                ));
            }
        }
        None => {
            return Err(Reason::User("Please set the bot channel".to_string()));
        }
    }
}

