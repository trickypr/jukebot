use std::time::Duration;

use color_eyre::eyre::Result;
use serenity::builder::{CreateCommand, CreateEmbed};

use super::{mpd, Response};

pub fn run() -> Response {
    match run_inner() {
        Ok(res) => res,
        Err(err) => Response::String(err.to_string()),
    }
}

fn run_inner() -> Result<Response> {
    let mut mpd = mpd()?;

    let status = mpd.status()?;
    Ok(match mpd.currentsong()? {
        Some(song) => Response::Embed(
            CreateEmbed::new()
                .title(song.title.unwrap_or("No title".to_string()))
                .description(song.artist.unwrap_or("No artist".to_string()))
                .field(
                    "Progress",
                    format!(
                        "{}/{}",
                        status.elapsed.unwrap_or(Duration::from_secs(0)).as_secs(),
                        song.duration.unwrap_or(Duration::from_secs(0)).as_secs()
                    ),
                    true,
                )
                .field(
                    "Repeat",
                    match status.repeat {
                        true => "on",
                        false => "off",
                    },
                    true,
                )
                .field(
                    "Random",
                    match status.random {
                        true => "on",
                        false => "off",
                    },
                    true,
                ),
        ),
        None => Response::String("Nothing playing".to_string()),
    })
}

pub fn register() -> CreateCommand {
    CreateCommand::new("playing").description("What is currently playing in the Common Room")
}
