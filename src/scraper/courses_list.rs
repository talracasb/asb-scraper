use scraper::{ElementRef, Html};
use serde::{Deserialize, Serialize};

use crate::{
    last,
    scraper::{single_elem_fragment, ValueNone},
    AnyError,
};

use super::Selectors;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CourseListitem {
    pub teacher: String,
    pub name: String,
    pub id: u32,
    pub absences: u32,
    pub tardies: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Courses {
    pub year_id: u16,
    pub courses: Box<[CourseListitem]>,
}

fn scrape_course(course: &ElementRef) -> Result<CourseListitem, AnyError> {
    let link_elem = single_elem_fragment(course, Selectors::CourseListEntryName.selector())?;
    let mut url = link_elem.attr("href").ok_or(ValueNone {})?.split('/');

    let id: u32 = last(&mut url).parse()?;

    let name = link_elem.inner_html();
    let teacher =
        single_elem_fragment(course, Selectors::CourseListEntryTeacher.selector())?.inner_html();

    // Seperate into seperate function eventually
    let absences: u32 = match course
        .select(Selectors::CourseListEntryAbsences.selector())
        .next()
    {
        Some(elem) => elem
            .text()
            .next()
            .ok_or(ValueNone {})?
            .trim()
            .strip_prefix("Absences: ")
            .ok_or(ValueNone {})?
            .parse()?,
        None => 0,
    };

    let tardies: u32 = match course
        .select(Selectors::CourseListEntryTardies.selector())
        .next()
    {
        Some(elem) => elem
            .text()
            .next()
            .ok_or(ValueNone {})?
            .trim()
            .strip_prefix("Tardies: ")
            .ok_or(ValueNone {})?
            .parse()?,
        None => 0,
    };

    Ok(CourseListitem {
        name: String::from(name.trim()),
        teacher: String::from(teacher.trim()),
        id,
        absences,
        tardies,
    })
}

pub fn scrape(html: &str, year_id: u16) -> Result<Courses, AnyError> {
    let doc = Html::parse_document(html);

    let courses: Result<Box<[CourseListitem]>, AnyError> = doc
        .select(Selectors::CourseListEntry.selector())
        .map(|course| scrape_course(&course))
        .collect();

    Ok(Courses {
        courses: courses?,
        year_id,
    })
}
