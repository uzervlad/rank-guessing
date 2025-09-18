import type { ApiBeatmap } from "./beatmap";
import type { ApiRequest } from "./request";

export type GetRequestResponse = {
  request: ApiRequest,
  beatmap: ApiBeatmap,
};