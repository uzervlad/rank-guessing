import { db } from "$lib/server/db";
import { requests } from "$lib/server/db/schema";
import { API } from "$lib/server/osu";
import type { RequestHandler } from "@sveltejs/kit";
import { eq } from "drizzle-orm";

export type SubmissionResponse = {
  username: string;
  rank: number;
  guess: number;
};

export const POST: RequestHandler = async ({ locals, request }) => {
  if (!locals.user?.isPlaying)
    return new Response("Unauthorized", { status: 401 });

  const { id, guess } = await request.json();

  if (!id || !guess)
    return new Response("Bad Request", { status: 400 });

  let [req] = await db.select()
    .from(requests)
    .where(eq(requests.id, id));

  if (!req)
    return new Response("Not Found", { status: 404 });

  const user = await API.getUser(req.player_id);

  // TODO: proper error handling?
  if (!user)
    return new Response("Internal Server Error", { status: 500 });

  await db.delete(requests)
    .where(eq(requests.id, id));

  return new Response(JSON.stringify({
    username: user.username,
    rank: user.statistics.global_rank,
    guess,
  }), {
    headers: {
      'Content-Type': 'application/json',
    },
  });
};