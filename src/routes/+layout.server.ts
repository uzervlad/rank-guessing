import { getMessage } from "$lib/server/message";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ locals }) => {
  return {
    user: locals.user,
    message: getMessage(),
  };
};