use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

use crate::last;

#[derive(Serialize, Deserialize)]
pub struct Home {
    pub day: String,
    pub student_name: String,
    pub student_id: String,
}

pub fn selector(selectors: &'static str) -> Selector {
    scraper::Selector::parse(selectors).unwrap()
}

pub fn single_elem_doc<'a>(doc: &'a Html, selectors: &'static str) -> ElementRef<'a> {
    let selector = selector(selectors);
    return doc.select(&selector).next().unwrap();
}

pub fn single_elem_fragment<'a>(doc: &'a ElementRef, selectors: &'static str) -> ElementRef<'a> {
    let selector = selector(selectors);
    return doc.select(&selector).next().unwrap();
}

pub fn home(html: &str) -> Home {
    let doc = scraper::Html::parse_document(html);

    let day = single_elem_doc(&doc, "span.day-text").inner_html();
    let name = single_elem_doc(&doc, "span.user-mini").inner_html();

    let regex: Regex = Regex::new(r"(.*?) \((\d*)\)").unwrap();

    let captures = regex.captures(&name).unwrap();

    Home {
        day,
        student_name: String::from(&captures[1]),
        student_id: String::from(&captures[2]),
    }
}

#[derive(Serialize, Deserialize)]
pub struct CourseListitem {
    pub teacher: String,
    pub name: String,
    pub id: u32,
    pub absences: u32,
    pub tardies: u32,
}

pub fn courses_list(html: &str) -> Box<[CourseListitem]> {
    let doc = scraper::Html::parse_document(html);

    let courses_selector = selector(".course-card-body");

    let absences_selector = selector(".absences-div > .attendance-text");
    let tardies_selector = selector(".tardies-div > .attendance-text");

    let courses: Box<[CourseListitem]> = doc
        .select(&courses_selector)
        .map(|course| {
            let link_elem = single_elem_fragment(&course, ".course-name-link");
            let mut url = link_elem.attr("href").unwrap().split('/');

            let id: u32 = last(&mut url).parse().unwrap();

            let name = link_elem.inner_html();
            let teacher = single_elem_fragment(&course, ".teacher-name").inner_html();

            let absences: u32 = match course.select(&absences_selector).next() {
                Some(elem) => {
                    let text = elem.text().next().unwrap();
                    text[text.find(':').unwrap() + 1..].trim().parse().unwrap()
                }
                None => 0,
            };

            let tardies: u32 = match course.select(&tardies_selector).next() {
                Some(elem) => {
                    let text = elem.text().next().unwrap();
                    text[text.find(':').unwrap() + 1..].trim().parse().unwrap()
                }
                None => 0,
            };

            CourseListitem {
                name: String::from(name.trim()),
                teacher: String::from(teacher.trim()),
                id,
                absences,
                tardies,
            }
        })
        .collect();

    courses
}

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

pub fn course(html: &str, id: u32, year_id: u32) -> Course {
    let doc = scraper::Html::parse_document(html);

    let assignment_selector = selector(".grade-card");

    let rc_selector = selector(".reporting-categories .assignment-standards .standard-li");
    let lb_selector = selector(".learning-behaviors .assignment-standards .standard-li");

    let title = single_elem_doc(&doc, ".course-title").inner_html();

    let regex = Regex::new(r"(.*?) \((.*?)\)").unwrap();
    let captures = regex.captures(&title).unwrap();

    let assignments: Box<[Assignment]> = doc
        .select(&assignment_selector)
        .map(|assignment| {
            let rc: Box<[Standard]> = assignment
                .select(&rc_selector)
                .map(|standard| Standard {
                    name: single_elem_fragment(&standard, ".standard-name").inner_html(),
                    grade: single_elem_fragment(&standard, ".score-bg")
                        .inner_html()
                        .parse()
                        .unwrap(),
                })
                .collect();

            let lb: Box<[Standard]> = assignment
                .select(&lb_selector)
                .map(|standard| Standard {
                    name: single_elem_fragment(&standard, ".standard-name").inner_html(),
                    grade: single_elem_fragment(&standard, ".learning-score")
                        .inner_html()
                        .parse()
                        .unwrap(),
                })
                .collect();

            Assignment {
                reporting_categories: rc,
                learning_behaviours: lb,
                term: single_elem_fragment(&assignment, ".assignment-term").inner_html(),
                name: single_elem_fragment(&assignment, ".card-title").inner_html(),
                date: single_elem_fragment(&assignment, ".assignment-date span").inner_html(),
            }
        })
        .collect();

    let attendence = single_elem_doc(&doc, ".attendance-text");

    let mut absences: u32 = 0;
    let mut tardies: u32 = 0;

    for text in attendence.text() {
        if let Some(count) = text.strip_prefix("Tardies: ") {
            tardies = count.parse().unwrap();
        } else if let Some(count) = text.strip_prefix("Absences: ") {
            absences = count.parse().unwrap();
        }
    }

    Course {
        assignments,
        name: String::from(captures[1].trim()),
        teacher: String::from(captures[2].trim()),
        id,
        year_id,
        tardies,
        absences,
    }
}
