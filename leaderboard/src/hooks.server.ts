import { sequence } from "@sveltejs/kit/hooks";
import { handleErrorWithSentry, sentryHandle } from "@sentry/sveltekit";
import * as Sentry from '@sentry/sveltekit';
import type { Handle } from "@sveltejs/kit";
import { svelteKitHandler } from "better-auth/svelte-kit";
import { auth } from "$lib/auth";
import { httpRequestCounter } from "$lib/server/metrics";

Sentry.init({
  dsn: 'https://a61312900a5b74c784e0b826af60a1ac@o4509610911334401.ingest.de.sentry.io/4509610913366096',
  tracesSampleRate: 1.0,
  spotlight: import.meta.env.DEV,
});

export const handle: Handle = sequence(
  sentryHandle(),
  ({ event, resolve }) => {
    return svelteKitHandler({ event, resolve, auth })
  },
  async ({ event, resolve }) => {
    const res = await resolve(event);
    httpRequestCounter.labels(event.request.method, event.url.pathname, res.status.toString()).inc();
    return res;
  }
);

export const handleError = handleErrorWithSentry();
