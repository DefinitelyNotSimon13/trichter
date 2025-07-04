import type { runsTable } from '$lib/server/db/schema/runs';

export type NewRun = typeof runsTable.$inferInsert;
export type Run = typeof runsTable.$inferSelect;
