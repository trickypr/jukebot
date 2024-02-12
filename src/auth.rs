use std::env;

use color_eyre::eyre::Result;
use serenity::{
    all::{GuildId, User},
    client::Context,
};

pub trait Auth {
    async fn can_use(&self, ctx: &Context) -> Result<bool>;
}

impl Auth for User {
    async fn can_use(&self, ctx: &Context) -> Result<bool> {
        let roles = env::var("ROLES")
            .expect("ROLES must be specified")
            .split(",")
            .map(|s| s.parse().expect("Each role must be an int"))
            .collect::<Vec<u64>>();

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("GUILD_ID must be set in the environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        for role in roles {
            if self.has_role(&ctx.http, guild_id, role).await? {
                return Ok(true);
            }
        }

        Ok(false)
    }
}
