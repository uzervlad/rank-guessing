<script lang="ts">
  import Button from '$lib/components/Button.svelte';

  const { data } = $props();
</script>

<main>
  <div class="settings">
    <a href="/play/toggle">
      <Button variant='secondary'>
        {data.open ? "Close submissions" : "Open submissions"}
      </Button>
    </a>
    <a href="/play/message">
      <Button>
        Change message
      </Button>
    </a>
  </div>

  <table>
    <thead>
      <tr>
        <th>ID</th>
        <th>User</th>
        <th>Beatmap</th>
        <th>Actions</th>
      </tr>
    </thead>
    <tbody>
      {#each data.requests as {requests, beatmaps}}
      <tr>
        <td>{requests.id}</td>
        <td>
          <a href={`https://osu.ppy.sh/u/${requests.player_id}`}>
            {requests.player_id}
          </a>
        </td>
        <td>
          <a href={`https://osu.ppy.sh/b/${beatmaps.id}`}>
            {beatmaps.artist} - {beatmaps.title} [{beatmaps.version}]
          </a>
        </td>
        <td>
          <Button variant='danger'>
            Delete
          </Button>
        </td>
      </tr>
      {/each}
    </tbody>
  </table>
</main>

<style lang="scss">
  .settings {
    padding: 8px 12px;
  }

  table {
    border-collapse: collapse;
  }
  th, td {
    padding: 4px 10px;
  }
  tbody > tr > td {
    border-top: 1px solid white;
  }
</style>