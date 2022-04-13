use std::{env, vec};

use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult, CommandError};

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

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    dotenv::dotenv().unwrap();
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
#[allowed_roles("Captian", "YarDev")]
#[only_in(guilds)]
#[sub_commands(start, end)]
async fn raid(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.reply(&ctx.http, "~yar raid <start {username}, end>").await?;
    Ok(())
}
#[command("start")]
#[allowed_roles("Captian", "YarDev")]
#[only_in(guilds)]
#[description("Start a raid. Provide the username that everyone will be joining the raid with.")]
/// Unwraps are allowed on the serde_json stuff because I can guarrentee that as long as roblox is avaiable
/// and there is one person in the game they will not panic
async fn start(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut mes = msg.reply(&ctx, "Starting a raid").await?;
    let username;
    if let Ok(user) = args.single::<String>(){
        username = user
    } else {
        mes.edit(&ctx.http, |m|m.content("~yar raid start {username}")).await?;
        return Ok(());
    }
    let id_url = constant::get_id_url(&username);
    let mut vec = vec![];
    let cookie = format!(".ROBLOSECURITY={}", std::env::var("ROBLO_SECURITY")?);
    let url = "https://web.roblox.com".parse::<reqwest::Url>()?;
    let jar = reqwest::cookie::Jar::default();
    jar.add_cookie_str(&cookie, &url);
    let client = reqwest::ClientBuilder::new()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:99.0) Gecko/20100101 Firefox/99.0")
        .cookie_store(true)
        .cookie_provider(std::sync::Arc::new(jar))
        .gzip(true)
        .build()?;
    let id_res = client.get(&id_url).send().await?;
    let text = id_res.text().await?;
    let id_json: serde_json::Value = serde_json::from_str(&text)?;
    let id_obj = id_json.as_object().unwrap();
    //The success value only exists in the response value if the request failed
    if id_obj.get("success").is_some() {
        mes.edit(&ctx, |m| m.content("Yar? Did you misspell the username put in")).await?;
        return Err(CommandError::from("Could not get username"));
    }
    let id = id_obj["Id"].as_u64().unwrap().to_string();
    let res = client.get(constant::get_avatar_url(&id)).send().await.unwrap();
    let avatar_url = res.url().as_str();
    for i in 0..constant::REQUEST_LIMIT {
        let url = constant::get_game_instances(constant::PLACE_ID, i);
        let res = client.get(url).send().await?;
        let json = res.text().await?;
        let json: serde_json::Value = serde_json::from_str(&json)?;
        let server = json.as_object().unwrap().clone();
        if server["Collection"].as_array().unwrap().is_empty(){
            break;
        }
        for x in server["Collection"].as_array().unwrap().clone() {
            for y in x.as_object().unwrap()["CurrentPlayers"].as_array().unwrap() {
                if y.as_object().unwrap()["Thumbnail"].as_object().unwrap()["Url"].as_str().unwrap() == avatar_url {
                    vec.push(x.as_object().unwrap()["Guid"].as_str().unwrap().to_string()); //Should really only be one
                }
            }
        }
    }

    if vec.is_empty() {
        mes.edit(&ctx.http, |m| {
            m.content("Yar? Are you online captian?")
        })
        .await?;
    } else {
        let join_url = constant::get_join_url(
            constant::PLACE_ID,
            &vec[0],
        );
        mes.edit(&ctx.http, |m| {
            m.content(format!(
                "@everyone Yar! Rading time (open url in web browser to join your captian):\n{join_url}"
            ))
        })
        .await?;
    }
    Ok(())
}

#[command("end")]
#[allowed_roles("Captian", "YarDev")]
#[only_in(guilds)]
#[description("End a raid")]
async fn end(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.reply(&ctx.http, "@everyone Yar! The raid is now over!")
        .await?;
    Ok(())
}
