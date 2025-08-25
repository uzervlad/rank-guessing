import { setMessage } from "$lib/server/message";
import { redirect, type RequestHandler } from "@sveltejs/kit";

export const POST: RequestHandler = async ({ locals, request }) => {
  if (!locals.isPlaying)
    return new Response("Unauthorized", { status: 401 });

  const { message } = await request.json();

  setMessage(message);

  throw redirect(302, '/play');
};