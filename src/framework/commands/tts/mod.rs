mod sample_rate;
mod speaker;

use super::{CommandResult, Ctx};
use serenity::all::Message;
use songbird::input::Input;
use speaker::Speaker;

use sample_rate::SampleRate;
use serde::Serialize;

use crate::api::endpoints::endpoints;
use crate::api::http_client;
use crate::framework::scenarios;
use crate::utils::types::Percentage;

#[derive(Serialize)]
pub struct Payload {
    pub text: String,
    pub speaker: Speaker,
    pub sample_rate: SampleRate,
    pub pitch: Percentage,
    pub rate: Percentage,
}

impl Default for Payload {
    fn default() -> Self {
        Payload {
            text: "".into(),
            speaker: Speaker::EnMan,
            sample_rate: SampleRate::High,
            pitch: Percentage::new(50).unwrap(),
            rate: Percentage::new(40).unwrap(),
        }
    }
}

pub async fn say_text(
    ctx: Ctx<'_>,
    pitch: Option<u8>,
    rate: Option<u8>,
    text: String,
    is_female_voice: bool,
) -> CommandResult {
    // identify if text is cyrillic or not
    let speaker = match (text.is_ascii(), is_female_voice) {
        (true, false) => Speaker::EnMan,
        (true, true) => Speaker::EnWoman,
        (false, false) => Speaker::RuMan,
        (false, true) => Speaker::RuWoman,
    };

    let voice = http_client()
        .get(endpoints().tts())
        .query(&Payload {
            text,
            speaker,
            pitch: Percentage::new_or_default(pitch),
            rate: Percentage::new_or_default(rate),
            sample_rate: SampleRate::High,
        })
        .send()
        .await?
        .bytes()
        .await?;

    scenarios::say_to_voice(ctx, Input::from(voice)).await?;

    ctx.reply("Saying...").await?.delete(ctx).await?;

    Ok(())
}
