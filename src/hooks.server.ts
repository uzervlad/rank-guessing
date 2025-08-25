import { env } from "$env/dynamic/private";
import type { Handle } from "@sveltejs/kit";
import jwt from 'jsonwebtoken';

export type Payload = {
  id: number,
  username: string,
};

export const handle: Handle = async ({ event, resolve }) => {
  const token = event.cookies.get("guess-token");

  event.locals.isPlaying = false;

  if (!token) {
    event.locals.user = null;
    return resolve(event);
  }

  try {
    const payload = jwt.verify(token, env.JWT_SECRET) as Payload;
    event.locals.user = payload;
    event.locals.isPlaying = payload.id === +env.PLAYER_ID;
  } catch {
    event.cookies.delete("guess-token", { path: '/' });
    event.locals.user = null;
  }

  return resolve(event);
}