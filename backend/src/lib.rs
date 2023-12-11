use std::error::Error;

use reqwest::Client;

pub mod handlers;
pub mod scraper;

pub type AnyError = Box<dyn Error>;

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
}
