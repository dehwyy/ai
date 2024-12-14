mod tts;

use super::scenarios;
use super::{CommandResult, Ctx};
use crate::api;
use poise::command;
use songbird::input::{Compose, YoutubeDl};

#[command(prefix_command, guild_only)]
pub(super) async fn play(
    ctx: Ctx<'_>,
    #[description = "The song to play"] song_url: String,
) -> CommandResult {
    let mut yt = YoutubeDl::new(api::http_client(), song_url.clone());
    let _track_handle = scenarios::say_to_voice(ctx, yt.clone().into()).await?;

    ctx.say(format!("{:?}", yt.aux_metadata().await?)).await?;

    Ok(())
}

/// Text to speech.
#[command(prefix_command, slash_command, guild_only)]
pub(super) async fn tts(
    ctx: Ctx<'_>,
    #[description = "Pitch of the voice, 50 recommended"]
    #[min = 0]
    #[max = 100]
    #[lazy]
    pitch: Option<u8>,
    #[description = "Speed of the voice, 40 recommended"]
    #[min = 0]
    #[max = 100]
    #[lazy]
    rate: Option<u8>,
    #[description = "Use male voice by default"]
    #[flag]
    is_female_voice: bool,
    #[description = "The text to speak"]
    #[rest]
    text: String,
) -> CommandResult {
    tts::say_text(ctx, pitch, rate, text, is_female_voice).await
}
