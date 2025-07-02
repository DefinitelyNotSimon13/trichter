import { betterAuth } from "better-auth";
import { mongodbAdapter } from "better-auth/adapters/mongodb";
import { db } from './server/mongo';
import { admin } from "better-auth/plugins"
import { BETTER_AUTH_SECRET } from "$env/static/private";

export const auth = betterAuth({
	database: mongodbAdapter(db),
	secret: BETTER_AUTH_SECRET,
	emailAndPassword: {
		enabled: true
	},
	plugins: [
		admin()
	]

})
