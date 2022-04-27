#![allow(unused_variables)]
use crate::commands::BOT_COMMAND_CHECK;
use crate::commands::POINT_COMMAND_CHECK;
use crate::{
    models::{UserInsertPoint, UserQueryPoint},
    schema::points::dsl::{id, points, user_points},
};
use diesel::query_dsl::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::{r2d2::ManageConnection, QueryDsl};
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
};
use std::ops::Add;

use crate::SqliteClient;
#[group]
#[commands(point)]
pub struct Point;

#[command("point")]
#[only_in(guilds)]
#[sub_commands(add, take, list, get, set_channel)]
pub async fn point(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.reply(&ctx, "~yar point <add {user mention} {points to add}, take {user mentition} {points to take}, list, get {user mentition}>\nAdd and Take require the Point role").await?;
    Ok(())
}

#[command]
#[checks(point_command)]
#[allowed_roles("Point", "Captian")]
async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let arg: String = args.single()?;
    let arg2: i32 = args.single()?;
    log::info!("Adding {arg2} points to {arg}");
    let data = ctx.data.read().await;
    let conn_manager = data
        .get::<SqliteClient>()
        .ok_or("Could not get connection manager")?;
    let mut conn = conn_manager.connect()?;
    let user_update = UserInsertPoint {
        id: &arg,
        user_points: arg2,
    };
    diesel::insert_into(points)
        .values(user_update)
        .on_conflict(id)
        .do_update()
        .set(user_points.eq(user_points.add(&arg2)))
        .execute(&mut conn)?;
    msg.reply(&ctx, format!("Added {arg2} points to {arg}"))
        .await?;
    Ok(())
}
#[command]
#[checks(point_command)]
#[allowed_roles("Point", "Captian")]
async fn take(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let arg: String = args.single()?;
    let arg2: i32 = args.single()?;
    log::info!("Taking {arg2} points from {arg}");
    let data = ctx.data.read().await;
    let conn_manager = data
        .get::<SqliteClient>()
        .ok_or("Could not get connection manager")?;
    let mut conn = conn_manager.connect()?;
    let result = points
        .filter(id.eq(&arg))
        .first::<UserQueryPoint>(&mut conn)?;
    diesel::update(points.filter(id.eq(&arg)))
        .set(user_points.eq(result.user_points - arg2))
        .execute(&mut conn)?;
    msg.reply(&ctx, format!("Took {arg2} points from {arg}"))
        .await?;
    Ok(())
}
#[command]
#[checks(bot_command)]
async fn get(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let arg: String = args.single()?;
    log::info!("Getting {arg}'s points");
    let data = ctx.data.read().await;
    let conn_manager = data
        .get::<SqliteClient>()
        .ok_or("Could not get connection manager")?;
    let mut conn = conn_manager.connect()?;
    let result = points
        .filter(id.eq(arg))
        .limit(1)
        .load::<UserQueryPoint>(&mut conn)?;
    msg.reply(&ctx, result[0].user_points).await?;
    Ok(())
}
#[command]
#[checks(bot_command)]
async fn list(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    log::info!("Listing top 5 pirates");
    let data = ctx.data.read().await;
    let conn_manager = data
        .get::<SqliteClient>()
        .ok_or("Could not get connection manager")?;
    let mut conn = conn_manager.connect()?;
    let mut results: Vec<crate::models::UserQueryPoint> =
        points.order(user_points).limit(5).load(&mut conn)?;
    results.reverse();
    let mut string = String::new();
    string.push_str("The top 5 pirates are:\n");
    for item in results {
        string.push_str(&format!("{} - {}\n", item.id, item.user_points));
    }
    msg.reply(&ctx.http, string).await?;
    Ok(())
}

#[command("set channel")]
#[only_in(guilds)]
async fn set_channel(ctx: &Context, msg: &Message, _: Args) -> CommandResult{
    let data = ctx.data.read().await;
    let conn_manager = data
        .get::<SqliteClient>()
        .ok_or("Could not get connection manager")?;
    let mut conn = conn_manager.connect()?;
    crate::config::set("point_channel_id", &msg.channel_id.to_string(), &mut conn)?;
    Ok(())
}