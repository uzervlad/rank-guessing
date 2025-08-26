import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import { db } from "$lib/server/db";
import { beatmaps, requests } from "$lib/server/db/schema";
import { eq } from "drizzle-orm";
import { submissionsOpen } from "$lib/server/open";

export const load: PageServerLoad = async ({ locals }) => {
  if (!locals.isAdmin)
    throw redirect(302, '/');

  const reqs = await db.select()
    .from(requests)
    .innerJoin(beatmaps, eq(requests.beatmap_id, beatmaps.id));

  const open = submissionsOpen();

  return {
    requests: reqs,
    open,
  };
};