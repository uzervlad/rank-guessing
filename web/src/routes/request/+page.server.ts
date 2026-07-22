import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import type { ApiRequest } from "$lib/types/request";
import type { ApiBeatmap } from "$lib/types/beatmap";

export const load: PageServerLoad = async ({ locals, fetch, setHeaders }) => {
  setHeaders({
    'cache-control': 'no-store',
  });

  if (!locals.user) throw redirect(302, '/');

  const { request, beatmap } = await fetch('/api/request').then(r => r.json());
  const { request: lastRequest } = await fetch('/api/request/last').then(r => r.json());

  return {
    request: request as ApiRequest | null,
    beatmap: beatmap as ApiBeatmap | null,
    lastRequest: lastRequest as ApiRequest | null,
  };
};