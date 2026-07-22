<script lang="ts">
  import { onMount } from "svelte";
  import { Emotes } from "$lib/emotes";
  import { sseListen, type Reader } from "$lib/sse";
  import type { ApiRequest } from "$lib/types/request";
  import type { ApiBeatmap } from "$lib/types/beatmap";
  import type { GetRequestResponse } from "$lib/types/responses";
  import type { EventHandler } from "svelte/elements";
  import { headers, safeResponse, type SafeResponse } from "$lib/fetch";

  import Button from "$lib/components/Button.svelte";
  import Input from "$lib/components/Input.svelte";
  import CircleX from "@lucide/svelte/icons/circle-x";
  import CircleCheck from "@lucide/svelte/icons/circle-check";
  import CircleAlert from "@lucide/svelte/icons/circle-alert";
  import PartyPopper from "@lucide/svelte/icons/party-popper";
	import Info from "@lucide/svelte/icons/info";
  import Back from "$lib/components/Back.svelte";
  import Spoiler from "$lib/components/Spoiler.svelte";
  import Comment from "$lib/components/Comment.svelte";
  import Confetti from "$lib/components/Confetti.svelte";
  import Beatmap from "$lib/components/Beatmap.svelte";

  const { data } = $props();
  
  let confetti: Confetti;

  const emotes = new Emotes();

  let reader: Reader | undefined;

  let readySubmissions = $state(0);
  let totalSubmissions = $state(0);

  const handleStateUpdate = (state: any) => {
    readySubmissions = state.ready_submissions;
    totalSubmissions = state.total_submissions;
  };

  let idInput = $state<number | undefined>();
  let guess = $state<number | undefined>();

  let error = $state('');
  let request = $state<ApiRequest | null>(null);
  let beatmap = $state<ApiBeatmap | null>(null);

  let commentOpen = $state(false);

  let result = $state<SafeResponse<GuessResponse> | null>(null);

  const fetchRequest = async (id?: number) => {
    let url = id ? `/api/play/request?id=${id}` : '/api/play/request';

    commentOpen = false;
    result = null;

    const { response, error: err } = await fetch(url)
      .then(safeResponse<GetRequestResponse>);

    if (!response) {
      error = err;
      request = null;
      beatmap = null;
      return;
    }

    error = '';
    request = response!.request;
    beatmap = response!.beatmap;
  };

  const getRequestRandom = () => fetchRequest();
  const getRequestById = () => {
    fetchRequest(idInput);
  };

  type GuessResponse = {
    username: string;
    rank: number;
    guess: number;
  };

  const submitGuess: EventHandler<SubmitEvent> = async (ev) => {
    ev.preventDefault();

    if (!request || !guess) return;
    open(`https://osu.ppy.sh/users/${request.player_id}/osu`);

    result = await fetch(`/api/play/guess`, {
      method: "POST",
      ...headers,
      body: JSON.stringify({ player_id: request.player_id, guess })
    })
      .then(safeResponse<GuessResponse>);

    guess = undefined;
  };

  const deleteRequest = async () => {
    if (!request) return;

    await fetch('/api/play/request', {
      method: "DELETE",
      ...headers,
      body: JSON.stringify({ id: request.id }),
    });

    request = null;
    beatmap = null;
  };

  onMount(() => {
    emotes.init();

    (async () => {
      const response = await fetch('/api/state');
  
      if (!response.body) {
        throw "why????";
      }
  
      reader = response.body.getReader();
      sseListen(reader, handleStateUpdate);
    })();

    return () => reader?.cancel();
  });
</script>

<Back to ='/guess' />
<PartyPopper class="confetti" onclick={() => confetti.spawnBurst()} />

<Confetti bind:this={confetti} auto={!!result?.response && (result.response.rank === result.response.guess)} />

<div class="settings">
  <a href='/guess/rename'>
    <Button>
      Change title
    </Button>
  </a>
  <a href='/guess/end'>
    <Button variant='danger'>
      End session
    </Button>
  </a>
</div>

<span class="count">
  {totalSubmissions} submissions ({readySubmissions} ready)
</span>

<div class="fetch">
  <Button onclick={getRequestRandom}>
    Random
  </Button>
  <Input
    type="number"
    bind:value={idInput}
    placeholder="ID"
  />
  <Button onclick={getRequestById}>
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
      <Beatmap beatmap={beatmap} />
      <div class="analysis">
        <ul>
          <li>
          {#if request.client_state == 'stable'}
            <CircleCheck color="#32d232" /> <span title="Score was set in stable">Stable replay</span>
          {:else if request.client_state == 'lazer'}
            <CircleCheck color="#32d232" /> <span title="Score was set in lazer">Lazer replay</span>
          {:else}
            <CircleAlert color="yellow" /> <span title="Score was set with lazer-specific mods">Lazer replay (with lazer mods)</span>
          {/if}
          </li>

          <li>
          {#if request.user_state == 'same-user'}
            <CircleCheck color="#32d232" /> <span title="User ID in replay matches submitter's ID">User matches</span>
          {:else if request.user_state == 'other-user'}
            <CircleX color="red" /> <span title="User ID in replay doesn't match submitter's ID">User doesn't match</span>
          {:else if request.online_state !== 'available'}
            <CircleAlert color="yellow" /> <span title="[Stable] Unable to check user due to score not being available online">Can't verify user</span>
          {:else}
            <CircleAlert color="yellow" /> <span title="No user ID present, likely offline score">No user</span>
          {/if}
          </li>

          <li>
            {#if request.online_state == 'available'}
              <CircleCheck color="#32d232" /> <span title="Score ID is present in replay and is available online">Online score available</span>
            {:else if request.online_state == 'unavailable'}
              <CircleAlert color="yellow" /> <span title="Score ID is present in replay but unavailable online">Online score unavailable</span>
            {:else}
              <CircleX color="red" /> <span title="No score ID in replay, potentially offline score">No score ID</span>
            {/if}
          </li>

          {#if request.session_id !== data.session?.id}
            <li>
              <CircleAlert color="yellow" /> <span title="This replay was submitted during a previous session">Wrong session</span>
            </li>
          {/if}

					{#if request.additional_notes}
						<li>
							<Info color="cyan" /> <span>{request.additional_notes}</span>
						</li>
					{/if}

          {#if request.watched_at}
            <li>
              <CircleX color="red" /> <span title="This replay already has been guessed earlier">Watched previously</span>
            </li>
          {/if}
        </ul>
      </div>
    </div>
    <div>
      {#if !result}
        {#if request.comment && data.session?.allow_comments}
          <Spoiler title="Comment" bind:open={commentOpen}>
            <Comment
              comment={request.comment}
              getEmote={emotes.getEmote.bind(emotes)}
            />
          </Spoiler>
        {/if}

        <a href={`/api/play/replay/${request.id}`} target="_blank">
          <Button variant='secondary'>
            Download replay
          </Button>
        </a>
        <span>Your guess:</span>
        <form onsubmit={e => e.preventDefault()}>
          <Input
            type="number"
            bind:value={guess}
            placeholder="Rank"
          />
          <div class="submit">
            <Button variant='submit' type="submit" onclick={submitGuess}>
              Submit
            </Button>
            <!-- does this even need to exist? -->
            <Button variant='danger' type="button" onclick={deleteRequest}>
              Delete
            </Button>
          </div>
        </form>
      {:else if result.response}
        <img class="avatar" src={`https://a.ppy.sh/${request.player_id}`} alt="">
        <span class="username">{result.response.username}</span>
        <span>Rank #{result.response.rank}</span>
        {#if result.response.rank === result.response.guess}
          <span>You were spot on!</span>
        {:else}
          <span>You were off by {Math.abs(result.response.rank - result.response.guess)}</span>
        {/if}
      {:else if result.error}
        <div class="error">
          <CircleX />
          <span>Error during guess submission:</span>
          <span>{result.error}</span>
        </div>
      {/if}
    </div>
  </div>
{/if}

<style lang="scss">
  .settings {
    display: flex;
    gap: 8px;
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

  .request {
    display: flex;
    flex-wrap: wrap;
    max-width: 100vw;
    justify-content: center;
    align-items: center;
    gap: 16px;
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

  form {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .submit {
    display: flex;
    justify-content: center;
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

      span[title] {
        text-decoration: underline dotted #999 1px;
        cursor: help;
      }
    }
  }

  .error {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  :global(.confetti) {
    position: absolute;
    top: 12px;
    left: 40px;

    cursor: pointer;
  }
</style>