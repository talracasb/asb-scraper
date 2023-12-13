import type { Course } from '$src/app';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, params }) => {
	const resp = await fetch(
		`http://localhost:3000/api/courses/${params.student}/${params.year}/${params.id}`,
		{
			headers: {
				SessionID: import.meta.env.VITE_SESSION_ID
			}
		}
	);

	const json: Course = await resp.json();

	return json;
};
