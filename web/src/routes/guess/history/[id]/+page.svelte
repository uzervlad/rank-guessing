<script lang="ts">
  import Back from '$lib/components/Back.svelte';
  import Button from '$lib/components/Button.svelte';
  import Input from '$lib/components/Input.svelte';
  import { headers } from '$lib/fetch';

  import CircleX from '@lucide/svelte/icons/circle-x';

  let editingNoteId = $state(0);
  let editingNote = $state("");

  let savingNote = $state(false);

  const saveNote = async () => {
    savingNote = true;

    try {
      await fetch('/api/admin/notes', {
        method: "POST",
        ...headers,
        body: JSON.stringify({
          request_id: editingNoteId,
          note: editingNote,
        }),
      });
    } finally {
      savingNote = false;
      editingNoteId = 0;
      editingNote = "";
    }
  };

  let { data } = $props();
</script>

<Back to='/guess/history' />

<table>
  <thead>
    <tr>
      <th>ID</th>
      {#if data.user?.is_admin}
      <th>User ID</th>
      {/if}
      <th>Beatmap</th>
      <th>Submitted at</th>
      <th>Guessed at</th>
      <th>Guess</th>
      {#if data.user?.is_admin}
      <th>Actions</th>
      {/if}
    </tr>
  </thead>
  {#if data.response}
  <tbody>
    {#each data.response as { request, beatmap }}
      <tr>
        <td>
          <a href={`https://osu.ppy.sh/users/${request.player_id}/osu`} target="_blank">
            {request.id}
          </a>
        </td>
        {#if data.user?.is_admin}
        <td>{request.player_id}</td>
        {/if}
        <td>
          <a href={`https://osu.ppy.sh/b/${beatmap.id}`} target="_blank">
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
            #{request.guessed_rank} / #{request.real_rank}
          {/if}
        </td>
        {#if data.user?.is_admin}
        <td>
          <Button variant='danger'>Delete</Button>
          <Button variant='secondary' onclick={() => {
            if (editingNoteId === request.id) {
              editingNoteId = 0;
              editingNote = "";
            } else {
              editingNoteId = request.id;
              editingNote = request.additional_notes ?? "";
            }
          }}>
            Notes
          </Button>
        </td>
        {/if}
      </tr>
      {#if editingNoteId === request.id}
      <tr>
        <td class="note" colspan="7">
          <div class="note-editor">
            <Input
              bind:value={editingNote}
              type="text"
              placeholder="Additional note"
              style="width: 100%"
            />
            <div class="note-actions">
              <Button
                variant='submit'
                disabled={savingNote}
                onclick={saveNote}
              >
                Save
              </Button>
              <Button
                variant='secondary'
                disabled={savingNote}
                onclick={() => {
                  editingNoteId = 0;
                  editingNote = "";
                }}
              >
                Cancel
              </Button>
            </div>
          </div>
        </td>
      </tr>
      {/if}
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

  tbody > tr > td:not(.note) {
    border-top: 1px solid #aaa;
  }

  .note-editor {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
</style>