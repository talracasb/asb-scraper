use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

use crate::{
    scraper::{single_elem_fragment, ValueNone},
    AnyError,
};

use super::single_elem_doc;

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
    pub name: String,
    pub teacher: String,
    pub tardies: u32,
    pub absences: u32,
    pub assignments: Box<[Assignment]>,
}

fn scrape_standard(
    standard: &ElementRef,
    grade_selector: &Selector,
    name_selector: &Selector,
) -> Result<Standard, AnyError> {
    Ok(Standard {
        name: single_elem_fragment(&standard, &name_selector)?.inner_html(),
        grade: single_elem_fragment(&standard, &grade_selector)?
            .inner_html()
            .parse()?,
    })
}

fn scrape_assignment(
    assignment: &ElementRef,
    rc_selector: &Selector,
    rc_grade_selector: &Selector,
    standard_name_selector: &Selector,
    lb_selector: &Selector,
    lb_grade_selector: &Selector,
    assignment_term_selector: &Selector,
    assignment_title_selector: &Selector,
    assignment_date_selector: &Selector,
) -> Result<Assignment, AnyError> {
    let rc: Result<Box<[Standard]>, AnyError> = assignment
        .select(&rc_selector)
        .map(|standard| scrape_standard(&standard, &rc_grade_selector, &standard_name_selector))
        .collect();

    let lb: Result<Box<[Standard]>, AnyError> = assignment
        .select(&lb_selector)
        .map(|standard| scrape_standard(&standard, &lb_grade_selector, &standard_name_selector))
        .collect();

    Ok(Assignment {
        reporting_categories: rc?,
        learning_behaviours: lb?,
        term: single_elem_fragment(&assignment, &assignment_term_selector)?.inner_html(),
        name: single_elem_fragment(&assignment, &assignment_title_selector)?.inner_html(),
        date: single_elem_fragment(&assignment, &assignment_date_selector)?.inner_html(),
    })
}

pub fn scrape(html: &str, id: u32, year_id: u32) -> Result<Course, AnyError> {
    let doc = Html::parse_document(html);

    let assignment_selector = Selector::parse(".grade-card")?;
    let assignment_term_selector = Selector::parse(".assignment-term")?;
    let assignment_title_selector = Selector::parse(".card-title")?;
    let assignment_date_selector = Selector::parse(".assignment-date span")?;

    let rc_selector = Selector::parse(".reporting-categories .assignment-standards .standard-li")?;
    let lb_selector = Selector::parse(".learning-behaviors .assignment-standards .standard-li")?;

    let standard_name_selector = Selector::parse(".standard-name")?;
    let rc_grade_selector = Selector::parse(".score-bg")?;
    let lb_grade_selector = Selector::parse(".learning-score")?;

    let title = single_elem_doc(&doc, ".course-title")?.inner_html();

    let regex = Regex::new(r"(.*?) \((.*?)\)")?;
    let captures = regex.captures(&title).ok_or(ValueNone {})?;

    let assignments: Result<Box<[Assignment]>, AnyError> = doc
        .select(&assignment_selector)
        .map(|assignment| {
            scrape_assignment(
                &assignment,
                &rc_selector,
                &rc_grade_selector,
                &standard_name_selector,
                &lb_selector,
                &lb_grade_selector,
                &assignment_term_selector,
                &assignment_title_selector,
                &assignment_date_selector,
            )
        })
        .collect();

    let attendence = single_elem_doc(&doc, ".attendance-text");

    let mut absences: u32 = 0;
    let mut tardies: u32 = 0;

    for text in attendence?.text() {
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
        tardies,
        absences,
    })
}
