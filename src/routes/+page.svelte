<script lang="ts">
  import { env } from "$env/dynamic/public";

  const auth_url = `https://osu.ppy.sh/oauth/authorize?client_id=${env.PUBLIC_OSU_CLIENT_ID}&redirect_uri=${encodeURIComponent(env.PUBLIC_OSU_REDIRECT_URI)}&response_type=code&score=identify`;
</script>

<main>
  <h1>Login</h1>
  <a href={auth_url}>
    <button>Login with osu!</button>
  </a>
</main>

<style lang="scss">
  @use "sass:color";

  button {
    background: #5b00d1;
    outline: none;
    border: none;
    border-radius: 5px;

    padding: 9px 15px;

    color: white;

    font-size: 24px;
    font-weight: 600;

    cursor: pointer;

    &:hover {
      background: color.adjust(#5b00d1, $lightness: 5%);
    }
  }
</style>