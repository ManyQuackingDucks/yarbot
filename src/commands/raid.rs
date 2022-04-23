use crate::{commands::RAID_COMMAND_CHECK, SqliteClient};
use diesel::r2d2::ManageConnection;
use log::info;
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
};

use crate::constant;

#[group]
#[commands(raid)]
#[allowed_roles("Raid")]
pub struct Raid;

#[command("raid")]
#[only_in(guilds)]
#[sub_commands(start, end, set_channel)]
#[checks(raid_command)]
pub async fn raid(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.reply(&ctx.http, "~yar raid <start {username}, end>")
        .await?;
    Ok(())
}
#[command("start")]
#[only_in(guilds)]
#[checks(raid_command)]
#[description("Start a raid. Provide the username that everyone will be joining the raid with.")]
/// Unwraps are allowed on the serde_json stuff because I can guarrentee that as long as roblox is avaiable
/// and there is one person in the game they will not panic
pub async fn start(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    log::info!("Starting a raid");
    let data = ctx.data.read().await;
    let client = data
        .get::<crate::ReqwestClient>()
        .ok_or("Could not get https client")?;
    let username = if let Ok(user) = args.single::<String>() {
        user
    } else {
        msg.reply(&ctx.http, "~yar raid start {username}").await?;
        return Ok(());
    };
    let id_url = constant::get_id_url(&username);

    let id_res = client.get(&id_url).send().await?;
    let id_json: serde_json::Value = id_res.json().await?;
    log::trace!("{id_json}");
    let id_obj = id_json
        .as_object()
        .ok_or("Could not deserialize id_json as object")?;
    //The success value only exists in the response value if the request failed
    if id_obj.get("success").is_some() {
        msg.reply(&ctx, "Yar? Did you misspell the username put in?")
            .await?;
        return Ok(());
    }
    let id = id_obj["Id"]
        .as_u64()
        .ok_or("Could not get id from the username")?;
    let mut map = std::collections::HashMap::new();
    map.insert("userIds", vec![id]);
    let game_url_res = client
        .post(constant::get_game_url())
        .json(&map)
        .send()
        .await?;
    let game_url_json: serde_json::Value = game_url_res.json().await?;
    log::trace!("{game_url_json}");
    let user_json = game_url_json
        .as_object()
        .ok_or("Could not deserialize game_url_json as object")?["userPresences"]
        .as_array()
        .ok_or("Could not get userPresences as array")?;
    let json = user_json
        .get(0)
        .ok_or("Could not get the first value of userPresences")?;
    let res = async {
        let user_json = json
            .as_object()
            .ok_or("Could not deserialize user_json as object from user_json")?;
        let place_id = user_json["placeId"]
            .as_u64()
            .ok_or("Could not deserialize placeId as u64 from user_json")?
            .to_string();
        let game_id = user_json["gameId"]
            .as_str()
            .ok_or("Could not deserialize gameId as str from user_json")?;
        let game_name = user_json["lastLocation"]
            .as_str()
            .ok_or("Could not desrialize lastLocation as str from user_json")?;
        let join_url = constant::get_join_url(&place_id, game_id);
        msg.reply(
            &ctx.http,
            format!("@pirate Yar! Rading time! We're playing: {game_name}\n{join_url}"),
        )
        .await?;
        CommandResult::Ok(())
    }
    .await;
    match res {
        Ok(()) => (),
        Err(e) => {
            msg.reply(&ctx.http, "Yar? Are you online?").await?;
            log::error!("raid start - {e}");
            return Ok(());
        }
    }
    Ok(())
}

#[command("end")]
#[only_in(guilds)]
#[checks(raid_command)]
#[description("End a raid")]
pub async fn end(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.reply(&ctx.http, "@pirate Yar! The raid is now over!")
        .await?;
    info!("Raid ended");
    Ok(())
}

#[command("set channel")]
#[only_in(guilds)]
async fn set_channel(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let channel_id = msg.channel_id.to_string();
    let data = ctx.data.read().await;
    let conn_manager = data
        .get::<SqliteClient>()
        .ok_or("Could not get connection manager")?;
    let mut conn = conn_manager.connect()?;
    crate::config::set("raid_channel_id", &channel_id, &mut conn)?;
    msg.reply(&ctx.http, "Set raid channel").await?;
    Ok(())
}
