#![feature(let_chains)]
#![feature(async_fn_traits)]
#![feature(async_closure)]

mod commands;
mod config;
mod constants;
mod db;
mod log_checking;
mod log_upload;
mod macros;
mod mappings;
mod util;

use std::fs;

use config::get_config;
use config::Config;
use db::init_db;
use log_upload::check_for_logs;
use mappings::cache::MappingsCache;
use poise::FrameworkOptions;
use serenity::all::CreateMessage;
use serenity::all::Message;
use serenity::all::Ready;
use serenity::async_trait;
use serenity::prelude::*;
use sqlx::postgres::PgPoolOptions;

pub struct ConfigData;

impl TypeMapKey for ConfigData {
    type Value = Config;
}

pub struct MappingsCacheKey;

impl TypeMapKey for MappingsCacheKey {
    type Value = MappingsCache;
}

pub struct DatabaseKey;

impl TypeMapKey for DatabaseKey {
    type Value = db::Db;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, event: Ready) {
        println!("Bot ready! Logged in as {}", event.user.name);
    }

    async fn message(&self, ctx: Context, message: Message) {
        match check_for_logs(&ctx, &message, false, false).await {
            Ok(Some(edit)) => {
                let reply = CreateMessage::default()
                    .content(edit.0)
                    .embeds(edit.1)
                    .components(edit.2)
                    .reference_message(&message);
                if let Err(err) = message.channel_id.send_message(&ctx, reply).await {
                    println!("Error posting log upload: {err}");
                }
            }
            Ok(None) => {
                // no-op
            }
            Err(err) => {
                println!("Log uploading threw error: {err}");
            }
        };
    }
}

#[tokio::main]
async fn main() {
    let mut commands = vec![
        commands::general::register(),
        commands::quote::quote(),
        commands::quote::context_quote(),
        commands::version::version(),
        commands::check_logs::check_logs_normal(),
        commands::check_logs::check_logs_compact(),
        commands::modversion::modversion(),
        commands::update_deps::update_deps(),
        commands::yarn::yarn(),
        commands::yarn::cache_status(),
        commands::shortcut::modrinth(),
        commands::reminder::make_reminder(),
    ];
    commands.append(&mut commands::tags::load_tag_commands());

    let poise_options = FrameworkOptions {
        commands,
        on_error: |err| {
            Box::pin(async move {
                println!("{err}");
            })
        },
        ..Default::default()
    };

    init_db().await;

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Registering commands");
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(ConfigData {})
            })
        })
        .options(poise_options)
        .build();

    // Login with a bot token from the environment
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&get_config().token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");
    {
        let mut data_lock = client.data.write().await;
        data_lock.insert::<MappingsCacheKey>(MappingsCache::create());
    }

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {why:?}");
    }
}
