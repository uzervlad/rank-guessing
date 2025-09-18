import { redirect } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async ({ locals, fetch }) => {
  if (!locals.user?.is_guesser && !locals.user?.is_admin)
    throw new Response("Unauthorized", { status: 404 });

  await fetch('/api/session', {
    method: "DELETE",
  });
  
  throw redirect(302, '/guess');
};