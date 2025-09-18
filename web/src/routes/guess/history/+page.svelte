<script lang="ts">
  import Back from '$lib/components/Back.svelte';
  import Button from '$lib/components/Button.svelte';
  import CircleX from '@lucide/svelte/icons/circle-x';

  const { data } = $props();
</script>

<Back to='/guess' />

<h2>Sessions</h2>

<table>
  <thead>
    <tr>
      <th>ID</th>
      <th>Date</th>
      <th>Submissions</th>
      <th></th>
    </tr>
  </thead>
  {#if data.response}
  <tbody>
    {#each data.response as session}
    <tr>
      <td>{session.id}</td>
      <td>{new Date(session.started_at).toLocaleDateString()}</td>
      <td>{session.submissions} ({session.guesses} guessed)</td>
      <td>
        <!-- <a href={`/guess/history/${session.id}/guesses`}> -->
          <Button variant='secondary' disabled title="TODO">
            Guesses
          </Button>
        <!-- </a> -->
        <!-- <a href={`/guess/history/${session.id}/requests`}> -->
          <Button disabled title="TODO">
            Submissions
          </Button>
        <!-- </a> -->
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