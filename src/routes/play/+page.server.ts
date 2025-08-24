import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "../$types";
import { submissionsOpen } from "$lib/server/open";

export const load: PageServerLoad = async ({ locals }) => {
  if (!locals.user?.isPlaying) throw redirect(302, '/');

  const open = submissionsOpen();

  return {
    open,
  };
};