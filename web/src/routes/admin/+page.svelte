<script lang="ts">
  import Back from "$lib/components/Back.svelte";
  import Button from "$lib/components/Button.svelte";
</script>

<Back to='/' />

<div class="nav">
  <a href='/guess/history'>
    <Button>Sessions</Button>
  </a>
  <Button variant='secondary' disabled>Users</Button>
</div>

<style lang="scss">
  .nav {
    display: flex;
    gap: 8px;
  }
</style>