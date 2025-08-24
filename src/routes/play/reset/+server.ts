import { db } from "$lib/server/db";
import { requests } from "$lib/server/db/schema";
import { redirect, type Actions, type RequestHandler } from "@sveltejs/kit";
import { _updateSubmissions } from "../count/+server";

export const POST: RequestHandler = async ({ locals }) => {
  if (!locals.user?.isPlaying)
    return new Response("Unauthorized", { status: 401 });

  await db.delete(requests);

  _updateSubmissions();

  throw redirect(302, "/play");
};