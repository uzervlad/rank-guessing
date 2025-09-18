import type { Handle } from "@sveltejs/kit";

export const handle: Handle = async ({ event, resolve }) => {
  if (event.url.pathname.startsWith('/api'))
    return resolve(event);

  if (!event.cookies.get('guess-token')) {
    event.locals.user = null;
    return resolve(event);
  }

  const user = await event.fetch('/api/auth/me')
    .then(r => r.status === 200 ? r.json() : null);

  event.locals.user = user;

  return resolve(event);
};