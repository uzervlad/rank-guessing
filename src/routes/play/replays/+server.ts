import { db } from "$lib/server/db";
import { beatmaps, requests } from "$lib/server/db/schema";
import type { RequestHandler } from "@sveltejs/kit";
import { eq, sql } from "drizzle-orm";
import { _updateSubmissions } from "../count/+server";

export const GET: RequestHandler = async ({ url, locals }) => {
  if (!locals.user?.isPlaying)
    return new Response("Unauthorized", { status: 401 });

  const id = url.searchParams.get('id');

  let request: typeof requests.$inferSelect;

  if (id) {
    [request] = await db.select()
      .from(requests)
      .where(eq(requests.id, +id));
  } else {
    [request] = await db.select()
      .from(requests)
      .where(eq(requests.ready, true))
      .orderBy(sql`RANDOM()`)
      .limit(1);
  }

  if (!request)
    return new Response(JSON.stringify({
      error: id ? "Replay not found" : "No replays available"
    }), { status: 404 });

  let [beatmap] = await db.select()
    .from(beatmaps)
    .where(eq(beatmaps.id, request.beatmap_id));

  return new Response(JSON.stringify({
    request, beatmap
  }), {
    headers: {
      'Content-Type': 'application/json',
    },
  });
};

export const DELETE: RequestHandler = async ({ url, locals }) => {
  if (!locals.user?.isPlaying)
    return new Response("Unauthorized", { status: 401 });

  const id = url.searchParams.get('id');

  if (!id)
    return new Response("Bad Request", { status: 400 });

  await db.delete(requests)
    .where(eq(requests.id, +id));

  _updateSubmissions();

  return new Response();
};