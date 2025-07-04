import { collectDefaultMetrics, Counter, Registry } from 'prom-client';

export const registry = new Registry();

collectDefaultMetrics({ register: registry });

export const httpRequestCounter = new Counter({
	name: 'http_requests_total',
	help: 'Total number of HTTP requests',
	registers: [registry],
	labelNames: ['method', 'route', 'status']
});
