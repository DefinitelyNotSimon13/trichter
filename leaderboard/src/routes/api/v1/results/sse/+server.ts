import type { SessionResult } from '$lib/models/session_result';
import { resultEmitter } from '$lib/server/events';
import type { RequestHandler } from '../$types';
import { produce } from 'sveltekit-sse';

export const POST: RequestHandler = () => {
    let onNew: (entry: SessionResult) => void
    let onUpdate: (entry: SessionResult) => void

    return produce(async ({ emit }) => {

        const send = (eventName: string, data: SessionResult) => {
            emit(eventName, JSON.stringify(data));
        };

        onNew = (entry: SessionResult) => send('new-result', entry);
        onUpdate = (entry: SessionResult) => send('updated-result', entry);

        resultEmitter.on('new-result', onNew);
        resultEmitter.on('updated-result', onUpdate);
    }, {
        stop() {
            resultEmitter.off('new-result', onNew);
            resultEmitter.off('updated-result', onUpdate);
        }
    })
};
