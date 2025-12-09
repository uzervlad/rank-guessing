<script lang="ts">
  import Button from '$lib/components/Button.svelte';
  import Input from '$lib/components/Input.svelte';
  import { sseListen } from '$lib/sse';

  import type { DragEventHandler } from 'svelte/elements';

  import LoaderPinwheel from "@lucide/svelte/icons/loader-pinwheel";
  import Upload from "@lucide/svelte/icons/upload";
  import Clapperboard from "@lucide/svelte/icons/clapperboard";
  import CircleX from "@lucide/svelte/icons/circle-x";
  import Check from "@lucide/svelte/icons/check";
  import Ellipsis from "@lucide/svelte/icons/ellipsis";
    import { headers } from '$lib/fetch.js';

  const { data } = $props();

  let comment = $state(data.request?.comment ?? '');

  let updating = $state(false);
  let updateSuccess = $state(false);

  // svelte-ignore non_reactive_update
  let input: HTMLInputElement;

  let uploading = $state(false);
  let uploadMessage = $state('...');
  let uploadError = $state('');

  let hovering = $state(false);

  const onDropzoneClick = () => {
    input.dispatchEvent(new MouseEvent('click'));
  };

  const onDropzoneDragover: DragEventHandler<HTMLDivElement> = (ev) => {
    ev.preventDefault();
    hovering = true;
  };

  const onDropzoneDragleave: DragEventHandler<HTMLDivElement> = (ev) => {
    ev.preventDefault();
    hovering = false;
  };
  
  const onDropzoneDrop: DragEventHandler<HTMLDivElement> = (ev) => {
    ev.preventDefault();
    hovering = false;

    let files = ev.dataTransfer?.files;
    if (files) {
      handleUpload(files);
    }
  };
  
  const onInputChange = () => {
    if (!input.files) return;

    handleUpload(input.files);

    input.value = null!;
  };

  const handleUpload = async (files: FileList) => {
    const file = files[0];

    if (!file.name.endsWith(".osr")) {
      uploadError = 'File is not a replay';
      return;
    }

    const form = new FormData();
    form.append("replay", files[0]);
    form.append("comment", comment);

    uploading = true;

    try {
      const response = await fetch('/api/request', {
        method: "POST",
        body: form,
      });
  
      if (!response.body) {
        throw "why????";
      }

      const reader = response.body.getReader();
      sseListen(reader, handleUploadMessage);
    } catch (e) {
      console.log(e);
      uploadError = "Unknown error";
      uploading = false;
    }
  };

  const handleUploadMessage = (message: any) => {
    console.log(message);

    switch (message.type) {
      case 'info':
        uploadMessage = message.message;
        break;
      case 'done':
        location.reload();
        break;
      case 'error':
        uploadError = message.message;
        uploading = false;
        break;
    } 
  };

  const updateComment = async () => {
    updating = true;

    const response = await fetch('/api/request', {
      method: "PATCH",
      ...headers,
      body: JSON.stringify({ comment }),
    });

    updateSuccess = true;

    setTimeout(() => {
      updating = false;
      updateSuccess = false;
    }, 2500);
  };

  const cancelRequest = async () => {
    await fetch('/api/request', {
      method: "DELETE",
    });

    location.reload();  
  };
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
{#if !data.session}
  <h2>Submissions are closed</h2>
  <h2>Come back next stream!</h2>
{:else if data.request}
  <h2>You have submitted a replay</h2>

  <span class="request">Your ID is: <code>{data.request.id}</code></span>

  {#if data.request.watched_at}
    <span>Your replay was watched at {new Date(data.request.watched_at).toLocaleString()}</span>
  {:else}
    <div class="comment">
      <Input
        bind:value={comment}
        type="text"
        placeholder="Comment (optional)"
        maxlength="100"
        style="width: 100%"
        bind:disabled={updating}
      />
      <span class="input-footer">max 100 characters, please don't spoil</span>
      <span class="input-footer">supports FFZ/BTTV/7TV emotes</span>
    </div>

    <div class="buttons">
      <Button variant='secondary' bind:disabled={updating} onclick={updateComment}>
        {#if updating && updateSuccess}
          <Check style="margin-bottom: -6px;" />
        {:else if updating}
          <Ellipsis style="margin-bottom: -6px;" />
        {:else}
          Update comment
        {/if}
      </Button>
      <Button variant='danger' onclick={cancelRequest}>
        Cancel request
      </Button>
    </div>
  {/if}
{:else}
  <div class="message">{data.session.title}</div>
  <h2>Send a replay</h2>

  {#if data.session.allow_comments}
    <div class="comment">
      <Input
        bind:value={comment}
        type="text"
        placeholder="Comment (optional)"
        maxlength="100"
        style="width: 100%"
        bind:disabled={uploading}
      />
      <span class="input-footer">max 100 characters, please don't spoil</span>
      <span class="input-footer">supports FFZ/BTTV/7TV emotes</span>
    </div>
  {/if}

  <input
    bind:this={input}
    class="file-input"
    type="file"
    accept=".osr"
    onchange={onInputChange}
  />
  <div
    class="dropzone"
    class:hovering={hovering}
    onclick={onDropzoneClick}
    ondragover={onDropzoneDragover}
    ondragleave={onDropzoneDragleave}
    ondrop={onDropzoneDrop}
  >
    {#if !uploading}
      {#if uploadError != ''}
        <CircleX />
        {uploadError}
      {:else}
        {#if hovering}
          <Upload />
          <span>Upload replay...</span>
        {:else}
          <Clapperboard />
          <span>Drop your replay here</span>
        {/if}
      {/if}
    {:else}
      <LoaderPinwheel class="loader" />
      <span>{uploadMessage}</span>
    {/if}
  </div>
{/if}

<a href='/request/history'>
  <Button>
    Submission history
  </Button>
</a>

<style lang="scss">
  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  :global(.loader) {
    animation: spin 1.6s linear infinite;
  }

  .file-input {
    display: none;
  }

  h2, .request {
    padding: 0 8px;
    text-align: center;
  }
  
  .request {
    margin-bottom: 6px;
  }

  .message {
    padding: 8px 12px;
    background: #fff1;
    border: 1px solid white;
    border-radius: 6px;
  }

  .comment {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;

    width: min(500px, 100%);
    margin-bottom: 16px;
  }

  .buttons {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    // align-items: center;
    gap: 8px;
    margin-bottom: 8px;
  }

  .input-footer {
    font-size: 13px;
    color: #888;
  }

  .dropzone {
    min-width: 250px;
    min-height: 80px;

    margin-bottom: 16px;

    display: flex;
    justify-content: center;
    align-items: center;
    gap: 8px;

    border: 2px dashed white;
    border-radius: 6px;
    padding: 8px 12px;

    user-select: none;

    cursor: pointer;

    &:hover {
      background: #fff1;
    }
  }
</style>