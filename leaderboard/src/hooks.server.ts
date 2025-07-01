import { auth } from "$lib/auth";
import { connect } from "$lib/server/mongo";
import { svelteKitHandler } from "better-auth/svelte-kit";

connect()
	.then((): void => {
		console.log("MongoDB connection established");
	})
	.catch((e) => {
		console.error("MongoDB connection failed", e);
	})

export async function handle({ event, resolve }) {
	return svelteKitHandler({ event, resolve, auth });
}
