<script lang="ts">
  import Back from '$lib/components/Back.svelte';
  import CircleX from '@lucide/svelte/icons/circle-x';

  let { data } = $props();
</script>

<Back to='/request' />

<h2>Submissions</h2>

<table>
  <thead>
    <tr>
      <th>ID</th>
      <th>Beatmap</th>
      <th>Submitted at</th>
      <th>Guessed at</th>
      <th>Guess</th>
    </tr>
  </thead>
  {#if data.response}
  <tbody>
    {#each data.response as { request, beatmap }}
    <tr>
      <td>{request.id}</td>
      <td>
        <a href={`https://osu.ppy.sh/b/${beatmap.id}`}>
          {beatmap.artist} - {beatmap.title}
        </a>
      </td>
      <td>{new Date(request.submitted_at).toLocaleString()}</td>
      <td>
        {#if request.watched_at}
          {#if request.vod_link}
            <a href={request.vod_link}>{new Date(request.watched_at).toLocaleString()}</a>
          {:else}
            {new Date(request.watched_at).toLocaleString()}
          {/if}
        {/if}
      </td>
      <td>
        {#if request.watched_at}
          #{request.guessed_rank}
        {/if}
      </td>
    </tr>
    {/each}
  </tbody>
  {/if}
</table>

{#if data.error}
  <CircleX />
  {data.error}
{/if}

<style lang="scss">
  td, th {
    padding: 6px 12px;
    text-align: center;
  }

  table {
    border-collapse: collapse;
  }

  tbody > tr > td {
    border-top: 1px solid #aaa;
  }
</style>