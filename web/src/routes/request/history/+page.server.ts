import { safeResponse } from "$lib/fetch";
import type { ApiBeatmap } from "$lib/types/beatmap";
import type { ApiRequest } from "$lib/types/request";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch }) => {
  const { response, error } = await fetch('/api/request/list')
    .then(safeResponse<{ request: ApiRequest, beatmap: ApiBeatmap }[]>);

  return { response, error };
};