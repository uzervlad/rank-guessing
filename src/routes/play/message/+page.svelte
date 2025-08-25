<script lang="ts">
  import Button from "$lib/components/Button.svelte";
  import { onMount } from "svelte";

  let message = $state('');

  let { data } = $props();

  const handleSubmit = async (ev: SubmitEvent) => {
    ev.preventDefault();

    await fetch('/play/message', {
      method: 'POST',
      body: JSON.stringify({ message }),
    });

    location.href = "/play";
  };

  onMount(() => {
    message = data.message;
  });
</script>

<main>
  <h2>Change message</h2>
  <form method="post" onsubmit={handleSubmit}>
    <input type="text" bind:value={message} placeholder="Message">
    <Button>Submit</Button>
  </form>
</main>

<style lang="scss">
  input {
    min-width: 250px;
    height: 37px;

    background: #333;
    border: 1px solid #555;
    border-radius: 4px;

    text-align: center;

    &:focus {
      border-color: #888;
      outline: none;
    }
  }
</style>