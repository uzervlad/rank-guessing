import { db } from "$lib/server/db";
import { requests } from "$lib/server/db/schema";
import type { RequestHandler } from "@sveltejs/kit";
import { count, eq, sql } from "drizzle-orm";

let totalSubmissions = 0;
let readySubmissions = 0;

type Subscriber = () => void;
const subscribers: Set<Subscriber> = new Set();

export async function _updateSubmissions() {
  let [{ total, ready }] = await db.select({
    total: count(),
    ready: sql<number>`coalesce(sum(case when ${requests.ready} = true then 1 else 0 end), 0)`,
  })
    .from(requests);

  totalSubmissions = total;
  readySubmissions = ready;

  for (let send of subscribers) send();
}

export const GET: RequestHandler = async ({ request, locals }) => {
  if (!locals.isPlaying)
    return new Response("Unauthorized", { status: 401 });

  let send: Subscriber;

  const responseStream = new ReadableStream({
    start(controller) {
      send = function() {
        controller.enqueue(`${JSON.stringify({ count: totalSubmissions, ready: readySubmissions })}\n`);
      };

      send();

      subscribers.add(send);

      request.signal.addEventListener('abort', () => {
        subscribers.delete(send);
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