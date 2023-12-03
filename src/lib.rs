use reqwest::Client;

pub mod handlers;
pub mod scraper;

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
}
