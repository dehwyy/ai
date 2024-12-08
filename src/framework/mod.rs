use serenity::async_trait;
use songbird::{Event, EventContext, EventHandler};

#[derive(Default, Debug)]
pub struct AppData;

type Error = String;
type Fw = poise::Framework<AppData, Error>;

mod hooks {
    use super::{AppData, Error, Fw};
    use poise::BoxFuture;
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
}
mod commands {
    use super::{AppData, Error};
    use poise::command;
    use reqwest::Proxy;
    use songbird::{
        input::{Compose, Input, YoutubeDl},
        tracks::Track,
        Event, TrackEvent,
    };
    use tokio::runtime::Handle;

    type Ctx<'a> = poise::Context<'a, AppData, Error>;
    type CommandResult = Result<(), Error>;

    #[command(prefix_command)]
    pub(super) async fn play(
        ctx: Ctx<'_>,
        #[description = "The song to play"] song_url: String,
        // #[description = "The song to play"] song_url: String,
        // #[description = "The song to play"] song_url: String,
    ) -> CommandResult {
        let guild_id = ctx.guild_id().unwrap();

        let channel_id = ctx
            .guild()
            .unwrap()
            .voice_states
            .get(&ctx.author().id)
            .unwrap()
            .channel_id
            .unwrap();

        println!("{} {} <- guild id", guild_id, channel_id);

        let v = songbird::get(ctx.serenity_context()).await.unwrap().clone();
        let _ = v.join(guild_id, channel_id).await;

        let handler_lock = v.get(guild_id).unwrap();
        let mut handler = handler_lock.lock().await;
        handler.deafen(false).await.unwrap();
        handler.mute(false).await.unwrap();

        let c = reqwest::Client::builder().build().unwrap();
        let mut yt = YoutubeDl::new(c, song_url.clone());

        // Convert from AuxMeta to Track

        let hh = handler.play_input(yt.clone().into());

        hh.add_event(TrackEvent::Error.into(), super::Handler)
            .unwrap();

        ctx.say(format!("{:?}", hh.get_info().await.unwrap()))
            .await
            .unwrap();

        ctx.say(format!("{:?}", yt.aux_metadata().await.unwrap()))
            .await
            .unwrap();

        Ok(())
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                println!(
                    "Track {:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }

        None
    }
}

pub fn new() -> Fw {
    poise::Framework::builder()
        .setup(hooks::on_setup)
        .options(poise::FrameworkOptions {
            commands: vec![commands::play()],
            on_error: |err| Box::pin(async move { println!("{}", err) }),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                // edit_tracker: Some(todo!())
                ..Default::default()
            },
            ..Default::default()
        })
        .build()
}
