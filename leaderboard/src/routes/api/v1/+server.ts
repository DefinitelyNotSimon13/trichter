import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async ({ request }) => {
    console.log("Got request: ", request);
    return new Response(JSON.stringify({ msg: "Hello, there!" }), {
        headers: { 'Content-Type': 'application/json' }
    })
}
