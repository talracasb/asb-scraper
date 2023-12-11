import type { CoursesList } from '$src/app';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
	let resp = await fetch('http://localhost:3000/api/courses/list', {
		headers: {
			SessionID: 'poxx5e4zqile9bdesl1nlmjoli5aorsj'
		}
	});

	let json: CoursesList = await resp.json();

	return json;
};
