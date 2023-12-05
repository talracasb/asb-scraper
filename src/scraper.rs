pub mod course;
pub mod courses_list;
pub mod home;

use scraper::{ElementRef, Html, Selector};
use std::error::Error;

use crate::AnyError;

#[derive(Debug)]
pub struct ValueNone {}

impl std::fmt::Display for ValueNone {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl Error for ValueNone {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

pub fn single_elem_doc<'a>(
    doc: &'a Html,
    selectors: &'static str,
) -> Result<ElementRef<'a>, AnyError> {
    let selector = scraper::Selector::parse(selectors)?;
    let elem = doc.select(&selector).next().ok_or(ValueNone {})?;
    Ok(elem)
}

pub fn single_elem_fragment<'a>(
    doc: &'a ElementRef,
    selector: &Selector,
) -> Result<ElementRef<'a>, AnyError> {
    let elem = doc.select(&selector).next().ok_or(ValueNone {})?;
    Ok(elem)
}
