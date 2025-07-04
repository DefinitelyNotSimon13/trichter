import { pgTable, uuid, varchar, real } from 'drizzle-orm/pg-core';

export const runsTable = pgTable('runs', {
	id: uuid().primaryKey().defaultRandom(),
	name: varchar({ length: 255 }),
	duration: real().notNull(),
	rate: real().notNull(),
	volume: real().notNull()
});
