import type { Course } from '$src/app';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, params }) => {
	let resp = await fetch(
		`http://localhost:3000/api/courses/${params.student}/${params.year}/${params.id}`,
		{
			headers: {
				SessionID: 'poxx5e4zqile9bdesl1nlmjoli5aorsj'
			}
		}
	);

	let json: Course = await resp.json();

	return json;
};
