pub mod endpoints;

use std::sync::OnceLock;

static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

pub fn http_client() -> reqwest::Client {
    HTTP_CLIENT
        .get_or_init(|| reqwest::Client::builder().build().unwrap())
        .clone()
}
