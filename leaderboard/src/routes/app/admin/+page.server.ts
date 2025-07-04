import { error, redirect } from '@sveltejs/kit';
import { Role } from '$lib/models/roles';
import type { PageServerLoad } from './$types';
import { auth } from '$lib/auth';

export const load: PageServerLoad = async ({ request }) => {
	const data = await auth.api.getSession({
		headers: request.headers
	});

	if (!data?.user) {
		redirect(302, '/signin');
	}

	if (data.user.role !== Role.Admin) {
		error(403, {
			code: 403,
			message: 'Forbidden'
		});
	}

	return {};
};
