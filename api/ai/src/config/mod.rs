use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "TOKEN")]
    pub bot_token: String,

    #[envconfig(from = "TTS_API_URL")]
    pub tts_api_url: String,
}

impl Config {
    pub fn new() -> Self {
        if let Err(e) = dotenv::from_filename(".env") {
            eprintln!("Failed to load .env file: {}", e);
        };

        let Ok(config) = Config::init_from_env() else {
            panic!("Failed to load config");
        };

        config
    }
}
