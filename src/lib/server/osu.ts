import { env as private_env } from '$env/dynamic/private';
import { env } from '$env/dynamic/public';
import { Auth, Client } from 'osu-web.js';
import { Porro } from 'porro';

export const auth = new Auth(
  +env.PUBLIC_OSU_CLIENT_ID,
  private_env.OSU_CLIENT_SECRET,
  env.PUBLIC_OSU_REDIRECT_URI
);

const bucket = new Porro({
  bucketSize: 120,
  interval: 1000,
  tokensPerInterval: 1,
});

class APIClient {
  private client?: Client;
  private expires_at: number = Date.now();

  private async check() {
    await bucket.throttle();

    if (!this.client || this.expires_at < Date.now()) {
      let token = await auth.clientCredentialsGrant();

      this.client = new Client(token.access_token);
      this.expires_at = Date.now() + 1000 * token.expires_in;
    }
  }

  async getBeatmapById(id: number) {
    await this.check();

    return await this.client?.beatmaps.getBeatmap(id);
  }

  async getBeatmapByHash(hash: string) {
    await this.check();

    return await this.client?.beatmaps.lookupBeatmap({
      query: {
        checksum: hash,
      },
    });
  }

  async getScore(id: number) {
    await this.check();

    return await this.client?.getUndocumented(`scores/${id}`);
  }

  async getScoreLegacy(id: number) {
    await this.check();

    return await this.client?.getUndocumented(`scores/osu/${id}`);
  }

  async getUser(id: number) {
    await this.check();

    return await this.client?.users.getUser(id, {
      urlParams: {
        mode: 'osu',
      },
    });
  }
}

export const API = new APIClient();