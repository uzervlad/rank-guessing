import { db } from "$lib/server/db";
import { beatmaps, OnlineState, requests, UserState } from "$lib/server/db/schema";
import { API } from "$lib/server/osu";
import { analyzeReplay, anonymizeReplay } from "$lib/server/replay";
import type { RequestHandler } from "@sveltejs/kit";
import { eq } from "drizzle-orm";
import fs from "fs";
import { _updateSubmissions } from "../play/count/+server";

export const POST: RequestHandler = async ({ fetch, locals, request }) => {
  if (!locals.user) {
    return new Response("Unauthorized", { status: 401 });
  }

  const playerId = locals.user.id;

  const responseStream = new ReadableStream({
    async start(controller) {
      function sendEvent(event: any) {
        controller.enqueue(`${JSON.stringify(event)}\n`);
      }

      let requestId = 0;

      sendEvent({ message: 'Checking access...' });

      try {
        let reqs = await db.select()
          .from(requests)
          .where(eq(requests.player_id, locals.user!.id));

        if (reqs.length !== 0) {
          sendEvent({ error: 'You already sent a replay'});
          controller.close();
          return;
        }

        sendEvent({ message: 'Receiving file...' });

        const form = await request.formData();
        const file = form.get('replay') as File | null;

        if (!file) {
          sendEvent({ error: 'No replay uploaded' });
          controller.close();
          return;
        }

        sendEvent({ message: 'Checking replay...' });

        const buffer = Buffer.from(await file.arrayBuffer());

        const analysis = await analyzeReplay(buffer);
        
        if (analysis.replay.info.rulesetId !== 0) {
          sendEvent({ error: 'Only standard replays allowed' });
          controller.close();
          return;
        }

        const beatmap = await API.getBeatmapByHash(analysis.replay.info.beatmapHashMD5);

        if (!beatmap) {
          sendEvent({ error: 'Beatmap not found' });
          controller.close();
          return;
        }

        await db.insert(beatmaps)
          .values({
            id: beatmap.id,
            beatmapset_id: beatmap.beatmapset_id,
            artist: beatmap.beatmapset.artist,
            title: beatmap.beatmapset.title,
            creator: beatmap.beatmapset.creator,
            version: beatmap.version,
          }).onConflictDoUpdate({
            target: beatmaps.id,
            set: {
              beatmapset_id: beatmap.beatmapset_id,
              artist: beatmap.beatmapset.artist,
              title: beatmap.beatmapset.title,
              creator: beatmap.beatmapset.creator,
              version: beatmap.version,
            },
          });

        let onlineState = OnlineState.NotPresent;
        if (analysis.onlineId > 0) {
          onlineState = OnlineState.Unavailable;
          try {
            let score: any = analysis.replay.info.isLegacyScore
              ? await API.getScoreLegacy(analysis.onlineId)
              : await API.getScore(analysis.onlineId);
            analysis.userId = score.user_id;
            onlineState = OnlineState.Available;
          } catch (err) {
            // console.log(err);
            // unavailable?
          }
        }

        let userState = analysis.userId !== playerId
          ? analysis.userId ? UserState.OtherUser : UserState.NotPresent
          : UserState.SameUser;

        const [req] = await db.insert(requests)
          .values({
            player_id: playerId,
            beatmap_id: beatmap.id,
            client_state: analysis.clientState,
            user_state: userState,
            online_state: onlineState,
          })
          .returning({ id: requests.id });

        requestId = req.id;

        const anonymousBuffer = await anonymizeReplay(analysis.replay, req.id);

        sendEvent({ message: 'Saving replay...' });

        if (!fs.existsSync("replays/")) {
          await fs.promises.mkdir('replays/');
        }
        await fs.promises.writeFile(`replays/${req.id}.osr`, anonymousBuffer);

        sendEvent({ message: 'Finalizing...' });

        await db.update(requests)
          .set({ ready: true })
          .where(eq(requests.id, req.id));

        sendEvent({ message: 'Upload complete', done: true, id: req.id });

        _updateSubmissions();
      } catch (err) {
        console.log(err);
        sendEvent({ error: 'Error occured during upload' });
        if (requestId) {
          await db.delete(requests)
            .where(eq(requests.id, requestId));
        }
      } finally {
        controller.close();
      }
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