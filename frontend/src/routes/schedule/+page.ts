import type { Home, Schedule } from '$src/app';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
	let resp = await fetch('http://localhost:3000/api/schedule', {
		headers: {
			SessionID: 'poxx5e4zqile9bdesl1nlmjoli5aorsj'
		}
	});

	let json: Schedule = await resp.json();

	console.log(json);

	return json;
};
