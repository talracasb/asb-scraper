import type { CoursesList } from '$src/app';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
	const resp = await fetch('http://localhost:3000/api/courses/list', {
		headers: {
			SessionID: import.meta.env.VITE_SESSION_ID
		}
	});

	const json: CoursesList = await resp.json();

	return json;
};
