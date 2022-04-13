use std::{env, vec};

use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::{
    async_trait, client::bridge::gateway::GatewayIntents, framework::standard::StandardFramework,
    model::gateway::Ready,
};
#[group]
#[commands(raid)]
struct Basic;
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[allow(dead_code)]
mod constant;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let framework = StandardFramework::new().configure(|c| {
            c.with_whitespace(true)
                .delimiters(vec![", ", ","])
                .prefix("~yar")
                .allow_dm(false)
        })
        .group(&BASIC_GROUP);
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .intents(GatewayIntents::all())
        .await
        .expect("Err creating client");
    client.start_autosharded().await.unwrap();
}

#[command("raid")]
#[allowed_roles("Captian")]
#[only_in(guilds)]
#[sub_commands(start, end)]
async fn raid(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.reply(&ctx.http, "~yar raid <start, end>").await?;
    Ok(())
}
#[command("start")]
#[allowed_roles("Captian")]
#[only_in(guilds)]
#[description("Start a raid")]
/// Unwraps are allowed on the serde_json stuff because I can guarrentee that as long as roblox is avaiable
/// and there is one person in the game they will not panic
async fn start(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let mut mes = msg.reply(&ctx, "Starting a raid").await?;
    let mut vec = vec![];
    let mut servers = vec![];
    let mut threads = vec![];
    let client = reqwest::Client::new();
    println!("Starting raid");
    for i in 0..constant::REQUEST_LIMIT {
        let client = client.clone();
        threads.push(tokio::spawn(async move {
            let url = constant::get_game_instances(constant::PLACE_ID, i * 10);
            let res = client.get(url).send().await?;
    
            let res_json: serde_json::Value = serde_json::from_str(&res.text().await?)?;
            let server = res_json.as_object().unwrap()["Collection"].as_array().unwrap().clone();
            CommandResult::Ok(server)
        }));
    }
    println!("Joining threads");
    for i in threads{
        let mut x: Vec<serde_json::Value> = i.await??;
        servers.append(&mut x);
    }
    println!("Finding guid");
    for i in servers{
        for y in i.as_object().unwrap()["CurrentPlayers"].as_array().unwrap() {
            if y.as_object().unwrap()["Id"].as_str().unwrap() == constant::CAPTAIN_ID {
                vec.push(i.clone()); //Should really only be one
            }
        }
    }

    if vec.is_empty() {
        mes.edit(&ctx.http, |m| {
            m.content("Yar! I could not find the captain online!")
        })
        .await?;
    } else {
        let join_url = constant::get_join_url(
            constant::PLACE_ID,
            vec[0].as_object().unwrap()["Guid"].as_str().unwrap(),
        );
        mes.edit(&ctx.http, |m| {
            m.content(format!(
                "@everyone Yar! Rading time (open url in web browser to join captian):\n{join_url}"
            ))
        })
        .await?;
        mes.pin(&ctx.http).await?;
    }
    Ok(())
}

#[command("end")]
#[allowed_roles("Captian")]
#[only_in(guilds)]
#[description("End a raid")]
async fn end(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.reply(&ctx.http, "@everyone Yar! The raid is now over!")
        .await?
        .pin(&ctx.http)
        .await?;
    Ok(())
}
