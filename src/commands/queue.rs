use color_eyre::eyre::Result;
use serenity::builder::CreateCommand;

use super::{mpd, Response};

pub fn run() -> Response {
    match run_inner() {
        Ok(res) => res,
        Err(err) => Response::String(err.to_string()),
    }
}

fn run_inner() -> Result<Response> {
    let mut mpd = mpd()?;

    let songs = mpd.queue()?;
    let status = mpd.status()?;

    let current_song_index = if let Some(place) = status.song {
        place.pos
    } else {
        u32::MAX
    };

    let mut out = String::new();

    let mut index = 0;
    for song in songs {
        let marker = if current_song_index == index {
            "**".to_string()
        } else {
            "".to_string()
        };
        out.push_str(&format!(
            "{}. {marker}{} - {}{marker}\n",
            index + 1,
            song.title.unwrap_or("No title".to_string()),
            song.artist.unwrap_or("No artist".to_string())
        ));
        index += 1;
    }

    if out == "" {
        return Ok(Response::String("Empty queue".to_string()));
    }

    Ok(Response::String(out))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("queue").description("List the songs in the queue")
}
