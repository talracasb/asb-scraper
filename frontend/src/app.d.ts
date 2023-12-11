// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface Platform {}
	}
}

// /
export interface Home {
	day: string;
	student_name: string;
	student_id: number;
}

// /grades
export interface CoursesListEntry {
	teacher: string;
	name: string;
	id: number;
	absences: number;
	tardies: number;
}

export interface CoursesList {
	year_id: number;
	student_id: number;
	courses: CoursesListEntry[];
}

// /grades/[student]/[year]/[id]
export interface Standard {
	grade: number;
	name: string;
}

export interface Assignment {
	name: string;
	date: string;
	term: string;
	reporting_categories: Standard[];
	learning_behaviours: Standard[];
}

export interface Course {
	id: number;
	year_id: number;
	student_id: number;
	name: string;
	teacher: string;
	tardies: number;
	absences: number;
	assignments: Assignment[];
}

// /schedule

export interface Period {
	name: string,
	time: string,
	visible: boolean,
}

export interface Class {
	id: string,
	name: string,
	teacher: string,
	email: string,
	room: string,
	color: string,
}

export interface Schedule {
	student: string,
	term: string,
	homeroom: string,
	header: Period[]
	days: Class[][]
}
