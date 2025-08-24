import { Score, type APIMod } from "osu-classes";
import { ClientState } from "./db/schema";
import { ScoreDecoder, ScoreEncoder } from 'osu-parsers';

export type AnalysisResult = {
  replay: Score,
  clientState: ClientState;
  userId: number;
  onlineId: number;
};

function isModLazer(mod: APIMod): boolean {
  const stableMods = [
    "EZ", "NF", "HT",
    "HR", "SD", "PF", "DT", "NC", "HD", "FL",
    "RX", "AP",
  ];

  if (!stableMods.includes(mod.acronym)) {
    return true;
  }

  // TODO: add settings check when osu-classes is updated

  return false;
}

export async function analyzeReplay(buffer: Buffer): Promise<AnalysisResult> {
  const decoder = new ScoreDecoder();

  const replay = await decoder.decodeFromBuffer(buffer, true);

  let clientState = ClientState.Stable;

  if (!replay.info.isLegacyScore) {
    clientState = ClientState.LazerDefault;

    if (replay.info.apiMods.some(isModLazer)) {
      clientState = ClientState.LazerMods;
    } 
  }

  return {
    replay,
    clientState,
    userId: replay.info.userId,
    onlineId: replay.info.id,
  };
}

export async function anonymizeReplay(replay: Score, id: number): Promise<Buffer> {
  replay.info.userId = null as unknown as number; // TODO: remove typescript fuckery when osu-classes is updated
  replay.info.username = `${id}`;

  const encoder = new ScoreEncoder();

  const buffer = await encoder.encodeToBuffer(replay);

  return Buffer.from(buffer);
}