import { betterAuth } from "better-auth";
import { mongodbAdapter } from "better-auth/adapters/mongodb";
import { db } from './server/mongo';
import { admin } from "better-auth/plugins"

export const auth = betterAuth({
	database: mongodbAdapter(db),
	emailAndPassword: {
		enabled: true
	},
	plugins: [
		admin()
	]

})
