import { redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";
import { headers } from "$lib/fetch";

export const actions: Actions = {
  default: async ({ request, locals, fetch }) => {
    if (!locals.user?.is_guesser)
      return new Response("Unauthorized", { status: 401 });

    const data = await request.formData();
    const name = data.get('name');
    const allow_comments = !!data.get('allow_comments');

    await fetch('/api/session', {
      method: 'POST',
      ...headers,
      body: JSON.stringify({ name, allow_comments }),
    });

    throw redirect(302, '/guess/play');
  }
};