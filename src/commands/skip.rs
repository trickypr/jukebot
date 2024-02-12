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
    mpd.next()?;
    Ok(Response::String("Skipped".to_string()))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("skip").description("Skips the currently playing song")
}
