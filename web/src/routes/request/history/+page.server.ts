import { safeResponse } from "$lib/fetch";
import type { ApiRequest } from "$lib/types/request";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch }) => {
  const { response, error } = await fetch('/api/request/list').then(safeResponse<ApiRequest[]>);

  return { response, error };
};