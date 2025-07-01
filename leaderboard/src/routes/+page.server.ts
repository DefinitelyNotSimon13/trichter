import { resultEmitter } from "$lib/server/events";
import { updateName } from "$lib/server/mongo";
import type { Actions } from "@sveltejs/kit";

export const actions: Actions = {
	updateName: async ({ request }) => {
		console.log("Got action...");
		const form = await request.formData();

		const id = form.get('id') as string;
		const name = form.get('drinker-name') as string;
		console.log("Id:", id);
		console.log("Name:", name);

		if (!id || !name) {
			return;
		}

		const updated = await updateName(id, name).catch((e) => { console.error("Failed to update name:", e) });

		if (updated) {
			resultEmitter.emit('updated-result', updated)
		}


		return;
	}
}
