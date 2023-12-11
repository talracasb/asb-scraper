use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

use crate::{
    scraper::{single_elem_fragment, ValueNone},
    AnyError,
};

use super::{single_elem_doc, Selectors};

#[derive(Serialize, Deserialize)]
pub struct Standard {
    pub name: String,
    pub grade: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Assignment {
    pub name: String,
    pub date: String,
    pub term: String,
    pub reporting_categories: Box<[Standard]>,
    pub learning_behaviours: Box<[Standard]>,
}

#[derive(Serialize, Deserialize)]
pub struct Course {
    pub id: u32,
    pub year_id: u32,
    pub student_id: u32,
    pub name: String,
    pub teacher: String,
    pub tardies: u32,
    pub absences: u32,
    pub assignments: Box<[Assignment]>,
}

fn scrape_standard(standard: &ElementRef, grade_selector: &Selector) -> Result<Standard, AnyError> {
    Ok(Standard {
        name: single_elem_fragment(standard, Selectors::CourseStandardName.selector())?
            .inner_html(),
        grade: single_elem_fragment(standard, grade_selector)?
            .inner_html()
            .parse()?,
    })
}

fn scrape_assignment(assignment: &ElementRef) -> Result<Assignment, AnyError> {
    let rc: Result<Box<[Standard]>, AnyError> = assignment
        .select(Selectors::CourseRC.selector())
        .map(|standard| scrape_standard(&standard, Selectors::CourseRCGrade.selector()))
        .collect();

    let lb: Result<Box<[Standard]>, AnyError> = assignment
        .select(Selectors::CourseLB.selector())
        .map(|standard| scrape_standard(&standard, Selectors::CourseLBGrade.selector()))
        .collect();

    Ok(Assignment {
        reporting_categories: rc?,
        learning_behaviours: lb?,
        term: single_elem_fragment(assignment, Selectors::CourseAssignmentTerm.selector())?
            .inner_html(),
        name: single_elem_fragment(assignment, Selectors::CourseAssignmentTitle.selector())?
            .inner_html(),
        date: single_elem_fragment(assignment, Selectors::CourseAssignmentDate.selector())?
            .inner_html(),
    })
}

pub fn scrape(html: &str, id: u32, year_id: u32, student_id: u32) -> Result<Course, AnyError> {
    let doc = Html::parse_document(html);

    let title = single_elem_doc(&doc, Selectors::CourseTitle.selector())?.inner_html();

    let regex = Regex::new(r"(.*?) \((.*?)\)")?;
    let captures = regex.captures(&title).ok_or(ValueNone {})?;

    let assignments: Result<Box<[Assignment]>, AnyError> = doc
        .select(Selectors::CourseAssignment.selector())
        .map(|assignment| scrape_assignment(&assignment))
        .collect();

    let attendence = single_elem_doc(&doc, Selectors::CourseAttendence.selector())?;

    let mut absences: u32 = 0;
    let mut tardies: u32 = 0;

    for text in attendence.text() {
        if let Some(count) = text.strip_prefix("Tardies: ") {
            tardies = count.parse()?;
        } else if let Some(count) = text.strip_prefix("Absences: ") {
            absences = count.parse()?;
        }
    }

    Ok(Course {
        assignments: assignments?,
        name: String::from(captures[1].trim()),
        teacher: String::from(captures[2].trim()),
        id,
        year_id,
        student_id,
        tardies,
        absences,
    })
}
