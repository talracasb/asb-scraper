use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{scraper::ValueNone, AnyError};

use super::{single_elem_doc, Selectors};

#[derive(Serialize, Deserialize)]
pub struct Home {
    pub day: String,
    pub student_name: String,
    pub student_id: String,
}

pub fn scrape(html: &str) -> Result<Home, AnyError> {
    let doc = scraper::Html::parse_document(html);

    let day = single_elem_doc(&doc, Selectors::HomeDay.selector())?.inner_html();
    let name = single_elem_doc(&doc, Selectors::HomeName.selector())?.inner_html();

    let regex: Regex = Regex::new(r"(.*?) \((\d*)\)")?;

    let captures = regex.captures(&name).ok_or(ValueNone {})?;

    Ok(Home {
        day,
        student_name: String::from(&captures[1]),
        student_id: String::from(&captures[2]),
    })
}
