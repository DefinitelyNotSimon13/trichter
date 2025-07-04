import { betterAuth } from 'better-auth';
import { admin } from 'better-auth/plugins';
import { env } from '$env/dynamic/private';
import { drizzleAdapter } from 'better-auth/adapters/drizzle';
import { db } from '$lib/server/db';
import * as auth_schema from '$lib/server/db/schema/auth-schema';

export const auth = betterAuth({
	database: drizzleAdapter(db, {
		provider: 'pg',
		schema: {
			...auth_schema
		}
	}),
	secret: env.BETTER_AUTH_SECRET,
	emailAndPassword: {
		enabled: true
	},
	plugins: [admin()]
});
