import { db } from "$lib/server/db";
import { requests } from "$lib/server/db/schema";
import type { RequestHandler } from "@sveltejs/kit";
import { count, eq, sql } from "drizzle-orm";

let submissionsCount = 0;

type Subscriber = () => void;
const subscribers: Set<Subscriber> = new Set();

export async function _updateSubmissions() {
  let [{ c }] = await db.select({ c: count() })
    .from(requests)
    .where(eq(requests.ready, true));

  submissionsCount = c;

  for (let send of subscribers) send();
}

export const GET: RequestHandler = async ({ request, locals }) => {
  if (!locals.user?.isPlaying)
    return new Response("Unauthorized", { status: 401 });

  let send: Subscriber;

  const responseStream = new ReadableStream({
    start(controller) {
      send = function() {
        controller.enqueue(`${JSON.stringify({ count: submissionsCount })}\n`);
      };

      send();

      subscribers.add(send);
      // const keepAlive = setInterval(() => send(), 5000);

      request.signal.addEventListener('abort', () => {
        subscribers.delete(send);
        // clearInterval(keepAlive);
      });
    },
    cancel() {
      subscribers.delete(send);
    },
  });

  return new Response(responseStream, {
    headers: {
      'Content-Type': 'application/x-ndjson',
      'Cache-Control': 'no-store',
      'Connection': 'keep-alive',
    },
  });
};