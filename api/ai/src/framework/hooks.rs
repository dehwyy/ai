use super::{AppData, Error, Fw};
use poise::{BoxFuture, FrameworkError};
use serenity::all::{Context, GuildId, Ready};
use std::{future::Future, pin::Pin};

const GUILD_ID: GuildId = GuildId::new(1086230642657873992);
const GUILD_ID_2: GuildId = GuildId::new(1102253349316862083);

pub fn on_setup<'a>(
    ctx: &'a Context,
    _ready: &'a Ready,
    fw: &'a Fw,
) -> BoxFuture<'a, Result<AppData, Error>> {
    Box::pin(async move {
        let created_commands = poise::builtins::create_application_commands(&fw.options().commands);

        GUILD_ID.set_commands(ctx, created_commands.clone()).await?;
        GUILD_ID_2.set_commands(ctx, created_commands).await?;
        // poise::builtins::register_globally(ctx, &fw.options().commands).await?;
        Ok(AppData::default())
    })
}

pub fn on_error<'a>(
    err: FrameworkError<'_, AppData, Error>,
) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
    Box::pin(async move {
        //
        println!("{}", err)
    })
}
