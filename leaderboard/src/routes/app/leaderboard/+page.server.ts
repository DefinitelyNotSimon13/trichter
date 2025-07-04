import { db } from '$lib/server/db';
import { runsTable } from '$lib/server/db/schema/runs';
import { updateRunName } from '$lib/server/db/router/runs';
import { ServerEvent } from '$lib/models/events';
import { resultEmitter } from '$lib/server/events';
import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const runs = await db.select().from(runsTable);
	return {
		runs: runs
	};
};

export const actions: Actions = {
	updateName: async ({ request }) => {
		const form = await request.formData();

		const id = form.get('id') as string;
		const name = form.get('run-name') as string;

		if (!id || !name) {
			return;
		}

		const updated = await updateRunName(id, name);

		if (updated) {
			resultEmitter.emit(ServerEvent.RunUpdated, updated);
		}

		return;
	}
};
