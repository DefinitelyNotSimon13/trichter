import type { Run } from '$lib/models/run';
import { ServerEvent } from '$lib/models/events';
import { produce } from 'sveltekit-sse';
import { resultEmitter } from '$lib/server/events';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = () => {
	let onNew: (entry: Run) => void;
	let onUpdate: (entry: Run) => void;

	return produce(
		async ({ emit }) => {
			const send = (eventName: string, data: Run) => {
				emit(eventName, JSON.stringify(data));
			};

			onNew = (entry: Run) => send(ServerEvent.RunCreated, entry);
			onUpdate = (entry: Run) => send(ServerEvent.RunUpdated, entry);

			resultEmitter.on(ServerEvent.RunCreated, onNew);
			resultEmitter.on(ServerEvent.RunUpdated, onUpdate);
		},
		{
			stop() {
				resultEmitter.off(ServerEvent.RunCreated, onNew);
				resultEmitter.off(ServerEvent.RunUpdated, onUpdate);
			}
		}
	);
};
