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
    mpd.play()?;
    Ok(Response::String("Playing".to_string()))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("play").description("Force music to be playing")
}

