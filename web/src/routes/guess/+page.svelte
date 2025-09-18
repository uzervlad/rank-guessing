<script>
  import Button from "$lib/components/Button.svelte";

  const { data } = $props();
</script>

<div class="menu">
  {#if data.session}
    <a href='/guess/play'>
      <Button>
        Back to guessing
      </Button>
    </a>
    <a href='/guess/end'>
      <Button variant='danger'>
        End session
      </Button>
    </a>
  {:else}
    <a href='/guess/new'>
      <Button variant='submit'>
        New session
      </Button>
    </a>
  {/if}
  <a href='/guess/history'>
    <Button variant='secondary'>
      History
    </Button>
  </a>
</div>

<style lang="scss">
  .menu {
    display: flex;
    gap: 8px;
  }
</style>