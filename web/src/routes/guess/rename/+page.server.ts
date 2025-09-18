import { headers } from "$lib/fetch";
import { redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions: Actions = {
  default: async ({ request, locals, fetch }) => {
    if (!locals.user?.is_guesser)
      return new Response("Unauthorized", { status: 401 });

    const data = await request.formData();
    const name = data.get('name');

    await fetch('/api/session', {
      method: "PATCH",
      ...headers,
      body: JSON.stringify({ name }),
    }).then(r => r.text()).then(console.log);

    throw redirect(302, '/guess/play');
  },
};