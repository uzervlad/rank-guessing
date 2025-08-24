<script>
  import LoaderPinwheel from "@lucide/svelte/icons/loader-pinwheel";
</script>

<LoaderPinwheel class="loader" />

<style lang="scss">
  @keyframes load {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  :global(.loader) {
    animation: load 1.6s linear infinite;
  }
</style>