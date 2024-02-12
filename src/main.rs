use std::env;

use commands::{mpd, Response};
use serenity::all::GuildId;
use serenity::async_trait;
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use color_eyre::eyre::{Context as _, Result};

mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction.clone() {
            let reply = match command.data.name.as_str() {
                "album" => commands::album::run(&command.data.options()),
                "albums" => commands::albums::run(),
                "artist" => commands::artist::run(&command.data.options()),
                "artists" => commands::artists::run(),
                "clear" => commands::clear::run(),
                "pause" => commands::pause::run(),
                "play" => commands::play::run(),
                "playing" => commands::playing::run(),
                "queue" => commands::queue::run(),
                "rescan" => commands::rescan::run(),
                _ => Response::String("Command not implemented :(".to_string()),
            };

            reply.send(command, ctx).await;
        };
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("GUILD_ID must be set in the environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        guild_id
            .set_commands(
                &ctx.http,
                vec![
                    commands::album::register(),
                    commands::albums::register(),
                    commands::artist::register(),
                    commands::artists::register(),
                    commands::clear::register(),
                    commands::pause::register(),
                    commands::play::register(),
                    commands::playing::register(),
                    commands::queue::register(),
                    commands::rescan::register(),
                ],
            )
            .await
            .expect("Failed to register commands");
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv()?;

    // Force consume by default
    let mut mpd = mpd()?;
    mpd.consume(true)?;
    drop(mpd);

    let discord_token =
        env::var("DISCORD_TOKEN").context("Expected a DISCORD_TOKEN to be in the environment")?;

    let mut client = Client::builder(discord_token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .context("Creating client")?;

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }

    Ok(())
}
