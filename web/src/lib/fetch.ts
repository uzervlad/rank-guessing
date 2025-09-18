export type SafeResponse<T extends {}> = {
  response: T;
  error: undefined;
} | {
  response: undefined;
  error: string
};

export const safeResponse = async <T extends {}>(r: Response): Promise<SafeResponse<T>> => r.status === 200
  ? { response: await r.json(), error: undefined }
  : { response: undefined, error: await r.text() };

export const headers = {
  headers: {
    "Content-Type": "application/json",
  },
};