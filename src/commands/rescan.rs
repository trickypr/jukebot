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
    mpd.rescan()?;
    Ok(Response::String("Rescaned".to_string()))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("rescan").description("Rescan your library")
}
