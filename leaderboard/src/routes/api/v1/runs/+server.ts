import { logger } from '$lib/logger';
import { ServerEvent } from '$lib/models/events';
import { getAllRuns, saveRun } from '$lib/server/db/router/runs';
import { resultEmitter } from '$lib/server/events';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async () => {
	logger.info("retrieving all runs...");
	const runs = await getAllRuns();
	return new Response(JSON.stringify(runs), {
		headers: { 'Content-Type': 'application/json' }
	});
};

export const POST: RequestHandler = async ({ request }) => {
	const auth = request.headers.get('authorization') ?? '';
	const expected = 'Basic dHJpY2h0ZXI6c3VwZXItc2FmZS1wYXNzd29yZA==';

	logger.info("received request", request)

	if (auth !== expected) {
		logger.warn("received request with invalid authentication");
		return new Response(JSON.stringify({ success: false, error: 'Unauthorized' }), {
			status: 401,
			headers: {
				'Content-Type': 'application/json',
				'WWW-Authenticate': 'Basic realm="Secure Area"'
			}
		});
	}

	const data = await request.json();

	if (!data.rate || !data.duration || !data.volume) {
		return new Response(JSON.stringify({ success: false }), {
			status: 400,
			headers: { 'Content-Type': 'application/json' }
		});
	}

	const createdRun = await saveRun(data);
	logger.info("created new run");

	resultEmitter.emit(ServerEvent.RunCreated, createdRun);

	return new Response(JSON.stringify({ success: true }), {
		headers: { 'Content-Type': 'application/json' }
	});
};
