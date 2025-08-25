import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ locals }) => {
  if (!locals.user) return;

  throw redirect(302, locals.isPlaying ? '/play' : '/request');
};