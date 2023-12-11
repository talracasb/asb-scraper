use serde::{Deserialize, Serialize};

use crate::{scraper::ValueNone, AnyError};

use super::Selectors;

#[derive(Serialize, Deserialize)]
pub struct Menu {
    pub title: String,
    pub href: String,
}

#[derive(Serialize, Deserialize)]
pub struct GradeMenus {
    normal: Menu,
    vegan: Menu,
}

#[derive(Serialize, Deserialize)]
pub struct LunchMenus {
    pub ecc: GradeMenus,
    pub general: GradeMenus,
}

pub fn scrape(html: &str) -> Result<LunchMenus, AnyError> {
    let doc = scraper::Html::parse_document(html);

    let mut links = doc.select(Selectors::LunchMenu.selector());

    let mut next = || -> Result<Menu, ValueNone> {
        let val = links.next().ok_or(ValueNone {})?;

        links.next(); // Skip spanish, because it's spanish.

        Ok(Menu {
            title: val.inner_html(),
            href: String::from(val.attr("href").ok_or(ValueNone {})?),
        })
    };

    Ok(LunchMenus {
        ecc: GradeMenus {
            normal: next()?,
            vegan: next()?,
        },
        general: GradeMenus {
            normal: next()?,
            vegan: next()?,
        },
    })
}
