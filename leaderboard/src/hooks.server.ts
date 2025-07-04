import { auth } from '$lib/auth';
import { httpRequestCounter } from '$lib/server/metrics';
import type { Handle } from '@sveltejs/kit';
import { svelteKitHandler } from 'better-auth/svelte-kit';



export const handle: Handle = async ({ event, resolve }) => {
	const metricsResolve = async (e: typeof event) => {
		const res = await resolve(e);
		httpRequestCounter.labels(e.request.method, e.url.pathname, res.status.toString()).inc();
		return res;
	};

	return svelteKitHandler({
		event,
		resolve: metricsResolve,
		auth
	});
};
