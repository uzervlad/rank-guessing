export type ApiSession = {
  id: number;
  title: string;
  started_at: string;
  ended_at: string | null;
};

export type ApiSessionExtended = ApiSession & {
  submissions: number;
  guesses: number;
};