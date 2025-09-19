import { safeResponse } from "$lib/fetch";
import type { ApiBeatmap } from "$lib/types/beatmap";
import type { ApiRequest } from "$lib/types/request";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch, params, url }) => {
  let requestsUrl = `/api/session/${params.id}`;

  if (url.searchParams.has('guessed')) {
    requestsUrl += '?guessed=1';
  }

  const { response, error } = await fetch(requestsUrl)
    .then(safeResponse<{ request: ApiRequest, beatmap: ApiBeatmap }[]>);

  return { response, error }
};