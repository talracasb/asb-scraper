use std::error::Error;

use reqwest::Client;

pub mod handlers;
pub mod scraper;

pub type AnyError = Box<dyn Error>;

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
}

pub fn last<'a>(iter: &mut impl Iterator<Item = &'a str>) -> &'a str {
    iter.next();

    let mut last = None;
    loop {
        let next = iter.next();

        if next.is_none() || next.unwrap().is_empty() {
            break;
        }

        last = next;
    }

    last.unwrap()
}
