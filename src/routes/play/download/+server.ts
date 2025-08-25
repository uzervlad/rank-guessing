import type { RequestHandler } from "@sveltejs/kit";

import fs from "fs";

export const GET: RequestHandler = async ({ url, locals }) => {
  if (!locals.isPlaying)
    return new Response("Unauthorized", { status: 401 });

  const id = url.searchParams.get('id');

  if (!id)
    return new Response("Not found", { status: 404 });

  if (!fs.existsSync(`replays/${id}.osr`))
    return new Response("Not found", { status: 404 });

  let stream = fs.createReadStream(`replays/${id}.osr`);

  const responseStream = new ReadableStream({
    start(controller) {
      stream.on('data', chunk => controller.enqueue(chunk));
      stream.on('end', () => controller.close());
      stream.on('error', (err) => controller.error(err));
    },
    cancel() {
      stream.destroy();
    },
  });

  return new Response(responseStream, {
    headers: {
      'Content-Type': 'x-osu-replay',
      'Content-Disposition': `attachment; filename="replay-${id}.osr"`,
    },
  });
};