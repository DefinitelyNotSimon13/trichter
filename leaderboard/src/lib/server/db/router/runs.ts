import type { NewRun, Run } from '$lib/models/run';
import { db } from '$lib/server/db';
import { runsTable } from '$lib/server/db/schema/runs';
import { eq } from 'drizzle-orm';

export async function saveRun(run: NewRun): Promise<Run> {
	const created = await db
		.insert(runsTable)
		.values({
			name: run.name ?? null,
			duration: run.duration,
			rate: run.rate,
			volume: run.volume
		})
		.returning();

	if (created.length === 0) {
		throw new Error(`Failed to create Run`);
	}

	return created[0];
}

export async function getAllRuns(): Promise<Run[]> {
	return db.query.runsTable.findMany();
}

export async function updateRunName(id: string, name: string): Promise<Run> {
	const updated = await db.update(runsTable).set({ name }).where(eq(runsTable.id, id)).returning();

	if (updated.length === 0) {
		throw new Error(`Failed to update Run with id ${id}`);
	}

	return updated[0];
}
