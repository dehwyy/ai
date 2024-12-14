use std::sync::OnceLock;

pub struct Endpoints {
    pub tts: String,
}

impl Endpoints {
    pub fn tts(&self) -> &str {
        &self.tts
    }
}

static ENDPOINTS: OnceLock<Endpoints> = OnceLock::new();

pub fn init_endpoints(endpoints: Endpoints) {
    let _ = ENDPOINTS.get_or_init(|| endpoints);
}

pub fn endpoints() -> &'static Endpoints {
    ENDPOINTS.get().unwrap()
}
