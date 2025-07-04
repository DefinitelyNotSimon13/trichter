import { env } from '$env/dynamic/private';
import { drizzle } from 'drizzle-orm/node-postgres';
import * as runsSchema from '$lib/server/db/schema/runs';

export const db = drizzle({
	connection: {
		connectionString: env.DATABASE_URL
	},
	schema: { ...runsSchema }
});
