<script lang="ts">
  import { env } from "$env/dynamic/public";
  
  import Upload from "@lucide/svelte/icons/upload";
  import Clapperboard from "@lucide/svelte/icons/clapperboard";
  import CircleX from "@lucide/svelte/icons/circle-x";

  import type { ChangeEventHandler, DragEventHandler, MouseEventHandler } from "svelte/elements";

  import Loader from "$lib/components/Loader.svelte"
    import { sseListen } from "$lib/sse.js";

  let { data } = $props();

  // svelte-ignore non_reactive_update
  let input: HTMLInputElement;

  let uploading = $state(false);
  let uploadMessage = $state("...");
  let uploadError = $state("");

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
    const form = new FormData();
    form.append("replay", files[0]);

    uploading = true;

    try {
      const response = await fetch('/request', {
        method: "POST",
        body: form,
      });
  
      if (!response.body) {
        throw "";
      }

      const reader = response.body.getReader();
      sseListen(reader, handleUploadMessage);
    } catch (e) {
      console.log(e);
      uploadError = "Unknown error";
      uploading = false;
    }
  };

  const handleUploadMessage = async (message: any) => {
    if (message.error) {
      uploadError = message.error;
      uploading = false;
      return;
    }
    if (message.done) {
      location.reload();
      return;
    }
    uploadMessage = message.message;
  };
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<main>
  <h1>{env.PUBLIC_TITLE}</h1>
  {#if !data.request}
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
        {#if uploadError != ""}
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
        <Loader />
        <span>{uploadMessage}</span>
      {/if}
    </div>
  {:else}
    <h2>You have submitted a replay</h2>

    <span>Your ID is: <code>{data.request.id}</code></span>
  {/if}
</main>

<style lang="scss">
  input {
    display: none;
  }

  .dropzone {
    min-width: 250px;
    min-height: 80px;

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