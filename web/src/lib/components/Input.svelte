<script lang="ts">
  let {
    value = $bindable(),
    disabled = $bindable(false),
    ...props
  } = $props();
</script>

<input class="input" bind:value={value} disabled={disabled} {...props}>

<style lang="scss">
  input {
    height: 37px;

    background: #333;
    border: 1px solid #555;
    border-radius: 4px;

    text-align: center;

    &:focus {
      border-color: #888;
      outline: none;
    }

    &::-webkit-outer-spin-button,
    &::-webkit-inner-spin-button {
      appearance: none;
    }
  }
</style>