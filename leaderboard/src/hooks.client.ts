import { handleErrorWithSentry, replayIntegration } from "@sentry/sveltekit";
import * as Sentry from '@sentry/sveltekit';

Sentry.init({
  dsn: 'https://a61312900a5b74c784e0b826af60a1ac@o4509610911334401.ingest.de.sentry.io/4509610913366096',
  tracesSampleRate: 1.0,
  replaysSessionSampleRate: 0.1,
  replaysOnErrorSampleRate: 1.0,
  integrations: [replayIntegration()],
});

export const handleError = handleErrorWithSentry();
