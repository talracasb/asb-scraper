pub mod course;
pub mod courses_list;
pub mod home;

use scraper::{ElementRef, Html, Selector};
use std::error::Error;
use tokio::sync::OnceCell;

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

pub fn single_elem_doc<'a>(doc: &'a Html, selector: &Selector) -> Result<ElementRef<'a>, AnyError> {
    let elem = doc.select(selector).next().ok_or(ValueNone {})?;
    Ok(elem)
}

pub fn single_elem_fragment<'a>(
    doc: &'a ElementRef,
    selector: &Selector,
) -> Result<ElementRef<'a>, AnyError> {
    let elem = doc.select(selector).next().ok_or(ValueNone {})?;
    Ok(elem)
}

static SELECTORS: OnceCell<Box<[Selector]>> = OnceCell::const_new();

#[derive(Copy, Clone)]
pub enum Selectors {
    HomeDay = 0,
    HomeName,
    CourseAssignment,
    CourseAssignmentTerm,
    CourseAssignmentTitle,
    CourseAssignmentDate,
    CourseRC,
    CourseLB,
    CourseStandardName,
    CourseRCGrade,
    CourseLBGrade,
    CourseTitle,
    CourseAttendence,
    CourseListName,
    CourseListEntry,
    CourseListTeacher,
    CourseListAbsences,
    CourseListTardies,
}

impl Selectors {
    pub fn as_str(&self) -> &str {
        match self {
            Selectors::HomeDay => "span.day-text",
            Selectors::HomeName => "span.user-mini",
            Selectors::CourseAssignment => ".grade-card",
            Selectors::CourseAssignmentTerm => ".assignment-term",
            Selectors::CourseAssignmentTitle => ".card-title",
            Selectors::CourseAssignmentDate => ".assignment-date span",
            Selectors::CourseRC => ".reporting-categories .assignment-standards .standard-li",
            Selectors::CourseLB => ".learning-behaviors .assignment-standards .standard-li",
            Selectors::CourseStandardName => ".standard-name",
            Selectors::CourseRCGrade => ".score-bg",
            Selectors::CourseLBGrade => ".learning-score",
            Selectors::CourseTitle => ".course-title",
            Selectors::CourseAttendence => ".attendance-text",
            Selectors::CourseListName => ".course-name-link",
            Selectors::CourseListEntry => ".course-card-body",
            Selectors::CourseListTeacher => ".teacher-name",
            Selectors::CourseListAbsences => ".absences-div > .attendance-text",
            Selectors::CourseListTardies => ".tardies-div > .attendance-text",
        }
    }

    pub fn selector(&self) -> &Selector {
        &SELECTORS.get().unwrap()[*self as usize]
    }
}

pub fn parse_selectors() {
    let selectors: Box<[Selector]> = (0..18)
        .map(|x: u8| {
            let selector = unsafe { std::mem::transmute::<u8, Selectors>(x) };

            Selector::parse(selector.as_str()).unwrap()
        })
        .collect();

    SELECTORS.set(selectors).unwrap();
}
