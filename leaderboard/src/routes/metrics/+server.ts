import { logger } from '$lib/logger';
import { registry } from '$lib/server/metrics';
import type { RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async ({ setHeaders }) => {
	logger.info("hello,there!");
	const metrics = await registry.metrics();
	setHeaders({
		'Content-Type': 'text/plain; version=0.0.4; charset=utf-8'
	});
	return new Response(metrics, { status: 200 });
};
