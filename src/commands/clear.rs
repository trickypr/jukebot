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
    mpd.clear()?;
    Ok(Response::String("Cleared".to_string()))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("clear").description("Clears the queue")
}
