use std::borrow::Cow;

use color_eyre::eyre::{ContextCompat, Result};
use mpd::{Query, Term};
use serenity::{
    all::{CommandOptionType, ResolvedOption, ResolvedValue},
    builder::{CreateCommand, CreateCommandOption},
};

use super::{mpd, Response};

pub fn run<'a>(options: &'a Vec<ResolvedOption<'a>>) -> Response {
    match run_inner(options) {
        Ok(res) => res,
        Err(err) => Response::String(err.to_string()),
    }
}

fn run_inner<'a>(options: &'a Vec<ResolvedOption<'a>>) -> Result<Response> {
    let mut mpd = mpd()?;

    let option = options
        .first()
        .and_then(|v| match v.value {
            ResolvedValue::String(val) => Some(val),
            _ => Some("Bad"),
        })
        .wrap_err("Artist was not specifed")?;

    mpd.findadd(Query::new().and(Term::Tag(Cow::from("artist")), option))?;

    Ok(Response::String("Probibly added artist".to_string()))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("artist")
        .description("Load an artist into the queue")
        .add_option(CreateCommandOption::new(
            CommandOptionType::String,
            "artist",
            "The name of the artist to load into the queue",
        ))
}
