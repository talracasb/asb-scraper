use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

use crate::{
    last,
    scraper::{single_elem_fragment, ValueNone},
    AnyError,
};

#[derive(Serialize, Deserialize)]
pub struct CourseListitem {
    pub teacher: String,
    pub name: String,
    pub id: u32,
    pub absences: u32,
    pub tardies: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Courses {
    pub year_id: u16,
    pub courses: Box<[CourseListitem]>,
}

fn scrape_course(
    course: &ElementRef,
    course_name_selector: &Selector,
    absences_selector: &Selector,
    tardies_selector: &Selector,
    teacher_name_selector: &Selector,
) -> Result<CourseListitem, AnyError> {
    let link_elem = single_elem_fragment(course, &course_name_selector)?;
    let mut url = link_elem.attr("href").ok_or(ValueNone {})?.split('/');

    let id: u32 = last(&mut url).parse()?;

    let name = link_elem.inner_html();
    let teacher = single_elem_fragment(&course, teacher_name_selector)?.inner_html();

    fn after_colon(elem: &ElementRef) -> Result<u32, AnyError> {
        let text = elem.text().next().ok_or(ValueNone {})?;
        Ok(text[text.find(':').ok_or(ValueNone {})? + 1..]
            .trim()
            .parse()?)
    }

    let absences: u32 = match course.select(&absences_selector).next() {
        Some(elem) => after_colon(&elem)?,
        None => 0,
    };

    let tardies: u32 = match course.select(&tardies_selector).next() {
        Some(elem) => after_colon(&elem)?,
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

    let course_name_selector = Selector::parse(".course-name-link")?;
    let courses_selector = Selector::parse(".course-card-body")?;
    let teacher_name_selector = Selector::parse(".teacher-name")?;

    let absences_selector = Selector::parse(".absences-div > .attendance-text")?;
    let tardies_selector = Selector::parse(".tardies-div > .attendance-text")?;

    let courses: Result<Box<[CourseListitem]>, AnyError> = doc
        .select(&courses_selector)
        .map(|course| {
            scrape_course(
                &course,
                &course_name_selector,
                &absences_selector,
                &tardies_selector,
                &teacher_name_selector,
            )
        })
        .collect();

    Ok(Courses {
        courses: courses?,
        year_id,
    })
}
