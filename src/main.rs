use std::env;
use std::time::Duration;

use auth::Auth as _;
use commands::{mpd, Response};
use mpd::{Song, State, Status};
use serenity::all::GuildId;
use serenity::all::OnlineStatus;
use serenity::async_trait;
use serenity::gateway::ActivityData;
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tokio;

use color_eyre::eyre::{Context as _, Result};

mod auth;
mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let can_access = command.user.can_use(&ctx).await.unwrap_or(false);

            let reply = if can_access {
                match command.data.name.as_str() {
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
                    "skip" => commands::skip::run(),
                    _ => Response::String("Command not implemented :(".to_string()),
                }
            } else {
                Response::String("You do not have access :(".to_string())
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
                    commands::skip::register(),
                ],
            )
            .await
            .expect("Failed to register commands");

        tokio::spawn(async move {
            fn online_from_status(status: Status) -> OnlineStatus {
                match status.state {
                    State::Stop | State::Pause => OnlineStatus::Idle,
                    State::Play => OnlineStatus::Online,
                }
            }

            fn activity_from_song(song: Option<Song>) -> Option<ActivityData> {
                song.map(|song| {
                    let song_title = song.title.unwrap_or("no title".to_string());
                    let song_artist = song.artist.unwrap_or("no artist".to_string());
                    let song_text = format!("{} - {}", song_title, song_artist);

                    ActivityData::listening(song_text)
                })
            }

            loop {
                let (activity, status) = match mpd() {
                    Ok(mut mpd) => match (mpd.currentsong(), mpd.status()) {
                        (Ok(current_song), Ok(status)) => {
                            (activity_from_song(current_song), online_from_status(status))
                        }
                        _ => (
                            Some(ActivityData::listening(String::from("error"))),
                            OnlineStatus::DoNotDisturb,
                        ),
                    },
                    Err(err) => {
                        println!("Error fetching mpd: {err}");
                        (
                            Some(ActivityData::listening(String::from("error"))),
                            OnlineStatus::DoNotDisturb,
                        )
                    }
                };

                ctx.set_presence(activity, status);
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        });
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv()?;

    // Force consume by default
    let mut mpd = mpd()?;
    mpd.consume(true)?;
    mpd.shuffle(..)?;
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
