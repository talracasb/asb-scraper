import type { Schedule } from '$src/app';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
	const resp = await fetch('http://localhost:3000/api/schedule', {
		headers: {
			SessionID: import.meta.env.VITE_SESSION_ID
		}
	});

	const json: Schedule = await resp.json();

	return json;
};
