mod commands;
mod hooks;
mod scenarios;

#[derive(Default, Debug)]
pub struct AppData;

type Error = crate::utils::AnyError;
type Fw = poise::Framework<AppData, Error>;
type Ctx<'a> = poise::Context<'a, AppData, Error>;
type CommandResult = Result<(), Error>;

pub fn new() -> Fw {
    poise::Framework::builder()
        .setup(hooks::on_setup)
        .options(poise::FrameworkOptions {
            commands: vec![commands::play(), commands::tts()],
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
