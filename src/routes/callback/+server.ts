import { env as private_env } from "$env/dynamic/private";
import { env } from "$env/dynamic/public";
import type { RequestHandler } from "@sveltejs/kit";
import jwt from 'jsonwebtoken';

export const GET: RequestHandler = async ({ url, fetch, cookies }) => {
  const code = url.searchParams.get('code');
  if (!code) return new Response("Missing authorization code", { status: 400 });

  const tokenData = await fetch('https://osu.ppy.sh/oauth/token', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      client_id: env.PUBLIC_OSU_CLIENT_ID,
      client_secret: private_env.OSU_CLIENT_SECRET,
      code,
      grant_type: 'authorization_code',
      redirect_uri: env.PUBLIC_OSU_REDIRECT_URI,
    }),
  }).then(r => r.json());

  const user = await fetch('https://osu.ppy.sh/api/v2/me', {
    headers: {
      'Authorization': `Bearer ${tokenData.access_token}`,
    },
  }).then(r => r.json());

  const isPlaying = user.id == private_env.PLAYER_ID;

  const jwtToken = jwt.sign({
    id: user.id, username: user.username, isPlaying,
  }, private_env.JWT_SECRET, {
    expiresIn: '7d',
  });

  cookies.set('guess-token', jwtToken, {
    path: '/',
    sameSite: 'lax',
    secure: false,
    maxAge: 60 * 60 * 24 * 7,
  });

  return new Response(null, {
    status: 302,
    headers: {
      'Location': isPlaying ? '/play' : '/request',
    },
  });
};