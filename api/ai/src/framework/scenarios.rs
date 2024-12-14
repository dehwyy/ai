use serenity::all::{ChannelId, GuildId};
use songbird::{input::Input, tracks::TrackHandle};

use super::*;

pub fn get_voice_channel(ctx: Ctx<'_>) -> Result<(GuildId, ChannelId), Error> {
    let guild = ctx.guild().ok_or("No guild.")?;

    let user_voice_channel = guild
        .voice_states
        .get(&ctx.author().id)
        .ok_or("User should be in channel to use this command")?;

    Ok((
        guild.id,
        user_voice_channel
            .channel_id
            .expect("?Should it be handled?"),
    ))
}

pub async fn say_to_voice(ctx: Ctx<'_>, input: Input) -> Result<TrackHandle, Error> {
    let (guild_id, channel_id) = get_voice_channel(ctx)?;

    let voice_cl = songbird::get(ctx.serenity_context())
        .await
        .expect("No songbird client")
        .clone();
    let handler_lock = voice_cl.join(guild_id, channel_id).await?;
    let mut handler = handler_lock.lock().await;

    Ok(handler.play_input(input))
}
