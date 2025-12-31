import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import type { ApiRequest } from "$lib/types/request";

export const load: PageServerLoad = async ({ locals, fetch }) => {
  if (!locals.user) throw redirect(302, '/');

  const { request } = await fetch('/api/request').then(r => r.json());
  const { request: lastRequest } = await fetch('/api/request/last').then(r => r.json());

  return {
    request: request as ApiRequest | null,
    lastRequest: lastRequest as ApiRequest | null,
  };
};