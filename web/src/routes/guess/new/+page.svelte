<script lang="ts">
  import Input from "$lib/components/Input.svelte";
  import Button from "$lib/components/Button.svelte";
  import Back from "$lib/components/Back.svelte";
</script>

<Back to='/guess' />

<form method="POST">
  <Input name="name" type="text" placeholder="Title" style="width: min(500px, 100%);" />
  <label for="allow_comments">
    <input type="checkbox" id="allow_comments" name="allow_comments" value="1" />
    Allow comments
  </label>
  <Button variant='submit' type='submit'>
    Start new session
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

  label {
    user-select: none;
  }
</style>