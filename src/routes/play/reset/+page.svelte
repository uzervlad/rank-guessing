<script>
  import Button from "$lib/components/Button.svelte";

  const deleteEverything = async () => {
    await fetch('/play/reset', { method: "POST" });

    location.href = "/play";
  };
</script>
<main>
  <h1>Are you sure?</h1>

  <div>
    <a href="/play">
      <Button variant='secondary' type='button'>
        No, take me back!
      </Button>
    </a>
    <Button variant='danger' onclick={deleteEverything}>
      Yes, delete everything
    </Button>
  </div>
</main>

<style lang="scss">
  h1 {
    text-align: center;
  }

  div {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    justify-content: center;
    padding: 8px;
  }
</style>