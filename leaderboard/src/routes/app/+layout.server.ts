import { error, redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import { auth } from '$lib/auth';

export const load: LayoutServerLoad = async ({ request }) => {
	const data = await auth.api.getSession({
		headers: request.headers
	});

	if (!data?.user) {
		redirect(302, '/signin');
	}

	if (!data.user.emailVerified) {
		error(403, {
			code: 403,
			message: 'Forbidden'
		});
	}

	return {
		user: data.user
	};
};
