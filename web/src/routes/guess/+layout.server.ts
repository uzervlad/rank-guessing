import type { ApiSession } from "$lib/types/session";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ fetch }) => {
  const { session } = await fetch('/api/session').then(r => r.json());

  return { session: session as ApiSession | null };
};