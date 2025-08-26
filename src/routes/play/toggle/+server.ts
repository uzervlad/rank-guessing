import { toggleSubmissions } from "$lib/server/open";
import { redirect, type RequestHandler } from "@sveltejs/kit";

export const GET: RequestHandler = async ({ locals }) => {
  if (!locals.isPlaying && !locals.isAdmin)
    return new Response("Unauthorized", { status: 401 });

  toggleSubmissions();

  throw redirect(302, locals.isPlaying ? '/play' : '/admin');
};