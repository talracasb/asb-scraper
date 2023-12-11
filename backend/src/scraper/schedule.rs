use regex::Regex;
use scraper::ElementRef;
use serde::{Deserialize, Serialize};

use crate::AnyError;

use super::{single_elem_doc, single_elem_fragment, Selectors, ValueNone};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Period {
    pub name: String,
    pub time: String,
    pub visible: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Class {
    pub id: String,
    pub name: String,
    pub teacher: String,
    pub email: String,
    pub room: String,
    pub color: String,
}

pub type Day = Box<[Option<Class>]>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Schedule {
    pub header: Box<[Period]>,
    pub days: Box<[Day]>,
    pub student: String,
    pub term: String,
    pub homeroom: String,
}

fn visible(elem: &ElementRef) -> bool {
    !matches!(elem.attr("data-period"), Some("3") | Some("6"))
}

pub fn scrape(html: &str) -> Result<Schedule, AnyError> {
    let doc = scraper::Html::parse_document(html);

    let header: Result<Box<[Period]>, AnyError> = doc
        .select(Selectors::ScheduleHeaders.selector())
        .map(|period| {
            Ok(Period {
                name: single_elem_fragment(&period, Selectors::ScheduleHeaderName.selector())?
                    .inner_html(),
                time: single_elem_fragment(&period, Selectors::ScheduleHeaderTime.selector())?
                    .inner_html(),
                visible: visible(&period),
            })
        })
        .collect();

    let days: Box<[Day]> = doc
        .select(Selectors::ScheduleDays.selector())
        .map(|day| {
            day.select(Selectors::ScheduleClasses.selector())
                .map(|class| {
                    let mut room_id = class
                        .select(Selectors::ScheduleClassRoomAndID.selector())
                        .next()?
                        .text();

                    let name = class
                        .select(Selectors::ScheduleClassName.selector())
                        .next()?;
                    let teacher = class.text().nth(2)?;
                    let email = class
                        .select(Selectors::ScheduleClassEmail.selector())
                        .next()?;

                    let color = class.attr("class")?.strip_prefix("schedule-color-")?;

                    Some(Class {
                        name: name.inner_html(),
                        teacher: String::from(teacher.trim()),
                        email: email.inner_html(),
                        room: String::from(room_id.next()?),
                        id: String::from(room_id.next()?),
                        color: String::from(color),
                    })
                })
                .collect::<Day>()
        })
        .collect();

    let student = single_elem_doc(&doc, Selectors::ScheduleStudent.selector())?.inner_html();

    let regex = Regex::new(r"(.+) \((.+)\) - (.*)")?;
    let captures = regex.captures(&student).ok_or(ValueNone {})?;

    Ok(Schedule {
        header: header?,
        days,
        homeroom: String::from(&captures[2]),
        student: String::from(&captures[1]),
        term: String::from(&captures[3]),
    })
}
