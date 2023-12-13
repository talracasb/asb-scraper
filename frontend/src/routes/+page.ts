import type { Home } from '$src/app';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
	const resp = await fetch('http://localhost:3000/api/home', {
		headers: {
			SessionID: import.meta.env.VITE_SESSION_ID
		}
	});

	const json: Home = await resp.json();

	return json;
};
