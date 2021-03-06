#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#[macro_use]
extern crate diesel;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
mod commands;
pub mod config;
#[allow(dead_code)]
mod constant;
mod hooks;
pub mod models;
#[cfg(feature = "replit")]
mod python_support;
pub mod schema;
use commands::point::POINT_GROUP;
use commands::raid::RAID_GROUP;
use diesel::r2d2::{ConnectionManager, ManageConnection};
use diesel::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use fern::colors::{Color, ColoredLevelConfig};
use hooks::{after_hook, dispatch_error};
use log::info;

use serenity::model::gateway::Activity;
use serenity::prelude::*;
use serenity::{async_trait, framework::standard::StandardFramework, model::gateway::Ready};

use std::{env, vec};

struct SqliteClient;
impl TypeMapKey for SqliteClient {
    type Value = diesel::r2d2::ConnectionManager<diesel::sqlite::SqliteConnection>;
}

struct ReqwestClient;
impl TypeMapKey for ReqwestClient {
    type Value = reqwest::Client;
}
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
        ctx.set_activity(Activity::playing("Pretty Much Evry Bordr Gaem"))
            .await;
    }
}

#[allow(dead_code)] //This is also a binary so this is required
fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(run());
}
async fn run() {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red)
        .trace(Color::Magenta)
        .debug(Color::Blue);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{colors}{date}[{target}][{level}{colors}] {message}\x1B[0m",
                colors = format_args!("\x1B[{}m", colors.get_color(&record.level()).to_fg_str()),
                date = chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                target = record.target(),
                level = record.level(),
                message = message,
            ));
        })
        .level(log::LevelFilter::Warn)
        .level_for("yarbot", log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
    dotenv::dotenv().ok();
    let connection_manager: ConnectionManager<SqliteConnection> =
        diesel::r2d2::ConnectionManager::new(env::var("DATABASE_URL").unwrap());
    let mut conn = connection_manager.connect().unwrap();
    conn.run_pending_migrations(MIGRATIONS).unwrap(); //Run migrations if required
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .delimiters(vec![", ", ",", " "])
                .prefix("~yar")
                .allow_dm(false)
        })
        .group(&RAID_GROUP)
        .group(&POINT_GROUP)
        .on_dispatch_error(dispatch_error)
        .after(after_hook);
    let mut client = Client::builder(&token, GatewayIntents::all())
        .event_handler(Handler)
        .framework(framework)
        .intents(GatewayIntents::all())
        .await
        .expect("Err creating client");
    {
        let mut data = client.data.write().await;
        data.insert::<SqliteClient>(connection_manager);
        let cookie = format!(
            ".ROBLOSECURITY={}",
            std::env::var("ROBLO_SECURITY").unwrap()
        );
        let url = "https://web.roblox.com".parse::<reqwest::Url>().unwrap();
        let url_2 = "https://presence.roblox.com"
            .parse::<reqwest::Url>()
            .unwrap();
        let jar = reqwest::cookie::Jar::default();
        jar.add_cookie_str(&cookie, &url);
        jar.add_cookie_str(&cookie, &url_2);
        let client = reqwest::ClientBuilder::new()
            .cookie_store(true)
            .cookie_provider(std::sync::Arc::new(jar))
            .gzip(true)
            .build()
            .unwrap();
        data.insert::<ReqwestClient>(client);
    }
    client.start_autosharded().await.unwrap();
}
