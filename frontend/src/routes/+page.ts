import type { Home } from '$src/app';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
	let resp = await fetch('http://localhost:3000/api/home', {
		headers: {
			SessionID: 'poxx5e4zqile9bdesl1nlmjoli5aorsj'
		}
	});

	let json: Home = await resp.json();

	return json;
};
