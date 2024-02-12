use std::borrow::Cow;

use color_eyre::eyre::Result;
use mpd::{Query, Term};
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

    let albums = mpd.list(&Term::Tag(Cow::from("album".to_string())), &Query::new())?;

    let mut out = String::new();

    let mut index = 0;
    for album in albums {
        out.push_str(&format!("{}. `{}`\n", index + 1, album));
        index += 1;
    }

    if out == "" {
        return Ok(Response::String("Empty albums".to_string()));
    }

    Ok(Response::String(out))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("albums").description("List the avaliable albums")
}
