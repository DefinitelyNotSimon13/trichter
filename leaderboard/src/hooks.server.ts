import { auth } from '$lib/auth';
import { httpRequestCounter } from '$lib/server/metrics';
import type { Handle } from '@sveltejs/kit';
import { svelteKitHandler } from 'better-auth/svelte-kit';
import * as Sentry from '@sentry/sveltekit';
import { sequence } from '@sveltejs/kit/hooks';
import { env } from '$env/dynamic/private';

Sentry.init({
	dsn: env.SENTRY_DSN,
	tracesSampleRate: 1
})

export const handleError = Sentry.handleErrorWithSentry();

export const handle: Handle = sequence(
	({ event, resolve }) => {
		return svelteKitHandler({ event, resolve, auth })
	},
	Sentry.sentryHandle(),
	async ({ event, resolve }) => {
		const res = await resolve(event);
		httpRequestCounter.labels(event.request.method, event.url.pathname, res.status.toString()).inc();
		return res;
	}
);
