import { safeResponse } from "$lib/fetch";
import { type ApiSessionExtended } from "$lib/types/session";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch }) => {
  const { response, error } = await fetch('/api/session/list').then(safeResponse<ApiSessionExtended[]>);

  return { response, error };
};