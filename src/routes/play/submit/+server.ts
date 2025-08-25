import { db } from "$lib/server/db";
import { requests } from "$lib/server/db/schema";
import { API } from "$lib/server/osu";
import type { RequestHandler } from "@sveltejs/kit";
import { eq } from "drizzle-orm";
import { _updateSubmissions } from "../count/+server";

export type SubmissionResponse = {
  username: string;
  rank: number;
  guess: number;
};

export const POST: RequestHandler = async ({ locals, request }) => {
  if (!locals.isPlaying)
    return new Response("Unauthorized", { status: 401 });

  const { player_id, guess } = await request.json();

  if (!player_id || !guess)
    return new Response("Bad Request", { status: 400 });

  const user = await API.getUser(player_id)

  // TODO: proper error handling?
  if (!user)
    return new Response("Internal Server Error", { status: 500 });

  await db.update(requests)
    .set({
      watched_at: new Date(),
    })
    .where(eq(requests.player_id, player_id));

  _updateSubmissions();

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