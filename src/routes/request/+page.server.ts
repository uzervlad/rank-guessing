import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "../$types";
import { db } from "$lib/server/db";
import { requests } from "$lib/server/db/schema";
import { eq } from "drizzle-orm";
import { submissionsOpen } from "$lib/server/open";

export const load: PageServerLoad = async ({ locals }) => {
  if (!locals.user) throw redirect(302, "/");

  let [request] = await db.select()
    .from(requests)
    .where(eq(requests.player_id, locals.user.id));

  let open = submissionsOpen();

  return {
    request,
    open,
  };
};