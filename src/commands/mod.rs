use std::net::TcpStream;

use color_eyre::eyre::Result;
use mpd::Client;
use serenity::{
    all::CommandInteraction,
    builder::{CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
};

pub mod album;
pub mod albums;
pub mod artist;
pub mod artists;
pub mod clear;
pub mod pause;
pub mod play;
pub mod playing;
pub mod queue;
pub mod rescan;

pub enum Response {
    String(String),
    Embed(CreateEmbed),
}

impl Response {
    pub async fn send(self, command: CommandInteraction, ctx: Context) {
        let mut data = CreateInteractionResponseMessage::new().ephemeral(true);

        match self {
            Response::String(string) => {
                data = data.content(string);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}")
                }
            }
            Response::Embed(embed) => {
                data = data.embed(embed);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }
}

pub fn mpd() -> Result<Client<TcpStream>> {
    Ok(Client::connect("127.0.0.1:6600")?)
}
