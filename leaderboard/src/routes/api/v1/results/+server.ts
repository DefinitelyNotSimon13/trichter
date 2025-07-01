import type { RequestHandler } from "./$types";
import { v4 as uuidv4 } from 'uuid';
import { resultEmitter } from "$lib/server/events";
import { getResults, saveResult } from "$lib/server/mongo";

export const GET: RequestHandler = async () => {
    const results = await getResults();
    return new Response(JSON.stringify(results), {
        headers: { 'Content-Type': 'application/json' }
    })
}

export const POST: RequestHandler = async ({ request }) => {
    const data = await request.json();
    data.id = uuidv4();

    if (!data.rate || !data.duration) {
        return new Response(JSON.stringify({ success: false }), {
            status: 400,
            headers: { 'Content-Type': 'application/json' }
        });
    }
    data.name = data.name ?? 'Unknown';

    await saveResult(data);

    resultEmitter.emit('new-result', data);


    return new Response(JSON.stringify({ success: true }), {
        headers: { 'Content-Type': 'application/json' }
    })
}
