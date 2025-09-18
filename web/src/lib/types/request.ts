type ClientState = 'stable' | 'lazer' | 'lazer-mods';
type OnlineState = 'available' | 'unavailable' | 'not-present';
type UserState = 'same-user' | 'other-user' | 'not-present';

export type ApiRequest = {
  id: number;
  player_id: number;
  session_id: number;
  beatmap_id: number;
  client_state: ClientState;
  online_state: OnlineState;
  user_state: UserState;
  ready: boolean;
  submitted_at: string;
  watched_at: string | null;
  guessed_rank: number | null;
  real_rank: number | null;
};