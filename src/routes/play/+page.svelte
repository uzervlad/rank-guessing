<script lang="ts">
  import { type beatmaps, type requests } from "$lib/server/db/schema";

  import { env } from "$env/dynamic/public";
  import Button from "$lib/components/Button.svelte";
  import CircleX from "@lucide/svelte/icons/circle-x";
  import CircleCheck from "@lucide/svelte/icons/circle-check";
  import CircleAlert from "@lucide/svelte/icons/circle-alert";

  import { type SubmissionResponse } from "./submit/+server";
  import type { EventHandler } from "svelte/elements";
  import { sseListen } from "$lib/sse";
  import { onMount } from "svelte";

  let { data } = $props();

  let inputId = $state('');
  let guess = $state('');

  let submissions = $state(0);
  let readySubmissions = $state(0);
  let submissionsError = $state('');

  let request = $state<typeof requests.$inferSelect | null>(null);
  let beatmap = $state<typeof beatmaps.$inferSelect | null>(null);
  let error = $state('');

  let result = $state<SubmissionResponse | null>(null);

  const fetchRequest = async (url: string) => {
    error = "";
    try {
      const { error: err, request: r, beatmap: b } = await fetch(url).then(r => r.json());
      if (err) {
        error = err;
        return;
      }
      request = r;
      beatmap = b;
      error = "";
    } catch {
      error = "Unknown error";
    }
  };

  const getRandom = () => fetchRequest('/play/replays');

  const getById = async () => {
    let id = parseInt(inputId);
    if (isNaN(id)) return;
    fetchRequest(`/play/replays?id=${id}`);
  };

  const submitGuess: EventHandler<SubmitEvent> = async (ev) => {
    ev.preventDefault();

    if (!request) return;
    open(`https://osu.ppy.sh/u/${request.player_id}`);

    result = await fetch(`/play/submit`, {
      method: 'POST',
      body: JSON.stringify({
        id: request.id,
        guess: +guess,
      }),
    }).then(r => r.json());
  };

  const deleteRequest = async () => {
    if (!request) return;

    await fetch(`/play/replays?id=${request.id}`, {
      method: 'DELETE',
    });

    request = null;
    beatmap = null;
  };

  const handleSubmissionsUpdate = (data: any) => {
    submissions = data.count;
    readySubmissions = data.ready;
  };

  onMount(() => {
    let reader: ReadableStreamDefaultReader;

    const listen = async () => {
      const response = await fetch('/play/count')
      if (!response.body) {
        submissionsError = "Unable to update submissions count";
        return;
      }

      reader = response.body.getReader();
      sseListen(reader, handleSubmissionsUpdate);
    };

    listen();

    return () => {
      if (reader) {
        reader.cancel();
      }
    };
  });

</script>

<main>
  <h1>{env.PUBLIC_TITLE}</h1>
  <h2>Guess the rank!</h2>

  <div class="settings">
    <a href="/play/toggle">
      <Button variant='secondary'>
        {#if data.open}
          Close submissions
        {:else}
          Open submissions
        {/if}
      </Button>
    </a>
    <a href="/play/reset">
      <Button variant='danger'>
        Delete all submissions
      </Button>
    </a>
  </div>

  <span class="count">
    {#if submissionsError !== ""}
      {submissionsError}
    {:else}
      {submissions} {submissions === 1 ? "submission" : "submissions"} ({readySubmissions} ready)
    {/if}
  </span>

  <div class="fetch">
    <Button onclick={getRandom}>
      Random
    </Button>
    <input
      type="number"
      bind:value={inputId}
      placeholder="ID"
    />
    <Button onclick={getById}>
      By ID
    </Button>
  </div>

  {#if error}
    <div class="error">
      <CircleX />
      {error}
    </div>
  {/if}

  {#if request && beatmap}
    <div class="request">
      <div>
        <div class="beatmap">
          <img class="cover" src={`https://assets.ppy.sh/beatmaps/${beatmap.beatmapset_id}/covers/cover.jpg`} alt="">
          <div class="info">
            <span class="title">{beatmap.artist} - {beatmap.title}</span>
            <span class="difficulty">[{beatmap.version}] by {beatmap.creator}</span>
          </div>
        </div>
        <div class="analysis">
          <ul>
            {#if request.client_state == 'stable'}
            <li>
              <CircleCheck color="#32d232" /> <span title="Score was set in stable">Stable replay</span>
            </li>
            {:else if request.client_state == 'lazer'}
            <li>
              <CircleCheck color="#32d232" /> <span title="Score was set in lazer">Lazer replay</span>
            </li>
            {:else}
            <li>
              <CircleAlert color="yellow" /> <span title="Score was set with lazer-specific mods">Lazer replay (with lazer mods)</span>
            </li>
            {/if}

            {#if request.user_state == 'same_user'}
            <li>
              <CircleCheck color="#32d232" /> <span title="User ID in replay matches submitter's ID">User matches</span>
            </li>
            {:else if request.user_state == 'other_user'}
            <li>
              <CircleX color="red" /> <span title="User ID in replay doesn't match submitter's ID">User doesn't match</span>
            </li>
            {:else if request.online_state !== 'available'}
            <li>
              <CircleAlert color="yellow" /> <span title="[Stable] Unable to check user due to score not being available online">Can't verify user</span>
            </li>
            {:else}
            <li>
              <CircleAlert color="yellow" /> <span title="No user ID present, likely offline score">No user</span>
            </li>
            {/if}

            {#if request.online_state == 'available'}
            <li>
              <CircleCheck color="#32d232" /> <span title="Score ID is present in replay and is available online">Online score available</span>
            </li>
            {:else if request.online_state == 'unavailable'}
            <li>
              <CircleAlert color="yellow" /> <span title="Score ID is present in replay but unavailable online">Online score unavailable</span>
            </li>
            {:else}
            <li>
              <CircleX color="red" /> <span title="No score ID in replay, potentially offline score">No score ID</span>
            </li>
            {/if}
          </ul>
        </div>
      </div>
      <div>
        {#if !result}
          <a href={`/play/download?id=${request.id}`}>
            <Button variant='secondary'>
              Download replay
            </Button>
          </a>
          <span>Your guess:</span>
          <form onsubmit={submitGuess}>
            <input
              type="number"
              bind:value={guess}
              placeholder="Rank"
            >
            <div>
              <Button variant='submit' type="submit">
                Submit
              </Button>
              <Button variant='danger' type="button" onclick={deleteRequest}>
                Delete
              </Button>
            </div>
          </form>
        {:else}
          <img class="avatar" src={`https://a.ppy.sh/${request.player_id}`} alt="">
          <span class="username">{result.username}</span>
          <span>Rank: {result.rank}</span>
          {#if result.rank === result.guess}
            <span>You were spot on!</span>
          {:else}
            <span>You were off by {Math.abs(result.rank - result.guess)}</span>
          {/if}
        {/if}
      </div>
    </div>
  {/if}
</main>

<style lang="scss">
  @use "sass:color";

  .settings {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    align-items: center;
    gap: 8px;
    margin: 4px;
  }

  .count {
    margin: 4px;
  }

  .fetch {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    align-items: center;
    gap: 8px;
    margin: 4px;

    @media screen and (max-width: 420px) {
      flex-direction: column;
    }
  }

  input {
    height: 37px;

    background: #333;
    border: 1px solid #555;
    border-radius: 4px;

    text-align: center;

    &:focus {
      border-color: #888;
      outline: none;
    }

    &::-webkit-outer-spin-button,
    &::-webkit-inner-spin-button {
      appearance: none;
    }
  }

  .error {
    background: #551f1f;
    border: 1px solid color.adjust(#551f1f, $lightness: 20%);
    border-radius: 6px;

    margin: 12px;
    padding: 12px;

    display: flex;
    justify-content: center;
    align-items: center;
    gap: 12px;
  }

  .request {
    display: flex;
    flex-wrap: wrap;
    max-width: 100vw;
    justify-content: center;
    align-items: center;
    margin: 4px;

    > div {
      width: 400px;

      display: flex;
      flex-direction: column;
      justify-content: center;
      align-items: center;
      gap: 8px;
    }

    @media screen and (max-width: 420px) {
      > div {
        width: calc(100vw - 16px);
      }
    }
  }

  .beatmap {
    background: #333;
    border: 1px solid #888;
    border-radius: 8px;
    overflow: hidden;

    .cover {
      max-width: 100%;
    }

    .info {
      margin: 10px;

      display: flex;
      flex-direction: column;
      gap: 6px;
    }
  }

  form {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  
  .avatar {
    max-width: 160px;
    border-radius: 8px;
  }

  .username {
    font-size: 24px;
    font-weight: 700;
  }

  ul {
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 4px;

    li {
      display: flex;
      align-items: center;
      gap: 8px;

      span {
        text-decoration: underline dotted #999 1px;
        cursor: help;
      }
    }
  }
</style>