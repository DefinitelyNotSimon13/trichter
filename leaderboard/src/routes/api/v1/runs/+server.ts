import { ServerEvent } from '$lib/models/events';
import { getAllRuns, saveRun } from '$lib/server/db/router/runs';
import { resultEmitter } from '$lib/server/events';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async () => {
	const runs = await getAllRuns();
	return new Response(JSON.stringify(runs), {
		headers: { 'Content-Type': 'application/json' }
	});
};

export const POST: RequestHandler = async ({ request }) => {
	const auth = request.headers.get('authorization') ?? '';
	const expected = 'Basic dHJpY2h0ZXI6c3VwZXItc2FmZS1wYXNzd29yZA==';


	if (auth !== expected) {
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

	resultEmitter.emit(ServerEvent.RunCreated, createdRun);

	return new Response(JSON.stringify({ success: true }), {
		headers: { 'Content-Type': 'application/json' }
	});
};
