import type { Actions } from "@sveltejs/kit";
import { auth } from "$lib/auth";

export const actions: Actions = {
	default: async ({ request }) => {
		console.log("Got action...");
		const form = await request.formData();

		const name = form.get('name') as string;
		const email = form.get('email') as string;
		const password = form.get('password') as string;
		console.log("Username:", name);
		console.log("Email:", email);
		console.log("Password:", password);

		const response = await auth.api.signUpEmail({
			body: {
				email,
				password,
				name,
				callbackURL: "/"
			},
			asResponse: true
		});

		return { success: response.status === 200 };
	}
}
