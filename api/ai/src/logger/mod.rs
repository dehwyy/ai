pub use tracing::*;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::prelude::*;

pub struct LoggerConfig {
    pub log_level: Level,
    pub with_sentry: bool,
    pub with_target: bool,
    pub with_file: bool,
    pub with_line_number: bool,
    pub with_ansi: bool,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        LoggerConfig {
            log_level: Level::INFO,
            with_sentry: true,
            with_ansi: true,
            with_file: true,
            with_target: true,
            with_line_number: true,
        }
    }
}

pub struct Logger;

impl Logger {
    pub fn init(config: LoggerConfig) {
        let fmt_subscriber = tracing_subscriber::FmtSubscriber::builder()
            .pretty()
            .with_target(config.with_target)
            .with_file(config.with_file)
            .with_line_number(config.with_line_number)
            .with_ansi(config.with_ansi)
            .with_timer(ChronoLocal::new("%H:%M:%S".to_owned()))
            .with_max_level(config.log_level)
            .finish();

        match config.with_sentry {
            true => fmt_subscriber
                .with(sentry::integrations::tracing::layer())
                .init(),
            false => fmt_subscriber.init(),
        }
    }
}
