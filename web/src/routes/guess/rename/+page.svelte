<script>
  import Back from "$lib/components/Back.svelte";
  import Button from "$lib/components/Button.svelte";
  import Input from "$lib/components/Input.svelte";

  const { data } = $props();

  let input = $state(data.session?.title);
</script>

<Back to='/guess/play' />

<form method="POST">
  <Input name="name" type="text" placeholder="Title" bind:value={input} style="width: min(500px, 100%);" />
  <Button variant='submit' type='submit'>
    Rename
  </Button>
</form>

<style lang="scss">
  form {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    width: 100%;
  }
</style>