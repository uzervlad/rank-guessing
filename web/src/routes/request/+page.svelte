<script lang="ts">
  import Button from '$lib/components/Button.svelte';
  import type { DragEventHandler } from 'svelte/elements';

  import LoaderPinwheel from "@lucide/svelte/icons/loader-pinwheel";
  import Upload from "@lucide/svelte/icons/upload";
  import Clapperboard from "@lucide/svelte/icons/clapperboard";
  import CircleX from "@lucide/svelte/icons/circle-x";
  import { sseListen } from '$lib/sse';

  const { data } = $props();

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
    <Button variant='danger' onclick={cancelRequest}>
      Cancel request
    </Button>
  {/if}
{:else}
  <div class="message">{data.session.title}</div>
  <h2>Send a replay</h2>

  <input
    bind:this={input}
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

  input {
    display: none;
  }

  h2, .request {
    padding: 0 8px;
    text-align: center;
  }

  .message {
    padding: 8px 12px;
    background: #fff1;
    border: 1px solid white;
    border-radius: 6px;
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