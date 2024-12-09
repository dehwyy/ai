#[derive(Default, Debug)]
pub struct AppData;

type Error = crate::utils::AnyError;
type Fw = poise::Framework<AppData, Error>;

mod hooks {
    use std::{future::Future, pin::Pin};

    use super::{AppData, Error, Fw};
    use poise::{BoxFuture, FrameworkError};
    use serenity::all::{Context, Ready};

    pub(super) fn on_setup<'a>(
        _ctx: &Context,
        _ready: &Ready,
        _fw: &'a Fw,
    ) -> BoxFuture<'a, Result<AppData, Error>> {
        Box::pin(async move {
            //
            Ok(AppData::default())
        })
    }

    pub(super) fn on_error<'a>(
        err: FrameworkError<'_, AppData, Error>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async move { println!("{}", err) })
    }
}

mod commands {
    use super::{AppData, Error};
    use poise::command;
    use songbird::input::{Compose, YoutubeDl};

    type Ctx<'a> = poise::Context<'a, AppData, Error>;
    type CommandResult = Result<(), Error>;

    #[command(prefix_command, guild_only)]
    pub(super) async fn play(
        ctx: Ctx<'_>,
        #[description = "The song to play"] song_url: String,
    ) -> CommandResult {
        let (guild_id, channel_id) = {
            let guild = ctx.guild().ok_or("No guild.")?;

            let user_voice_channel = guild
                .voice_states
                .get(&ctx.author().id)
                .ok_or("User should be in channel to use this command")?;

            (
                guild.id,
                user_voice_channel
                    .channel_id
                    .expect("?Should it be handled?"),
            )
        };

        let voice_cl = songbird::get(ctx.serenity_context()).await.unwrap().clone();
        let handler_lock = voice_cl.join(guild_id, channel_id).await?;
        let mut handler = handler_lock.lock().await;

        let mut yt = YoutubeDl::new(crate::static_clients::http_client(), song_url.clone());

        let _track_handle = handler.play_input(yt.clone().into());

        ctx.say(format!("{:?}", yt.aux_metadata().await?)).await?;

        Ok(())
    }
}

pub fn new() -> Fw {
    poise::Framework::builder()
        .setup(hooks::on_setup)
        .options(poise::FrameworkOptions {
            commands: vec![commands::play()],
            on_error: hooks::on_error,
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                // edit_tracker: Some(todo!())
                ..Default::default()
            },
            ..Default::default()
        })
        .build()
}
