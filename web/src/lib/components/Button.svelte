<script lang="ts">
  type ButtonVariant = 'primary' | 'secondary' | 'submit' | 'danger';

  let { children, disabled = $bindable(false), variant = 'primary' , ...props } = $props();
</script>

<button class={variant} class:disabled={disabled} disabled={disabled} {...props}>
  {@render children?.()}
</button>

<style lang="scss">
  @use "sass:color";

  button {
    outline: none;
    border: none;
    border-radius: 4px;

    padding: 7px 12px;

    font-size: 18px;
    font-weight: 500;

    cursor: pointer;
  }

  .disabled {
    cursor: not-allowed;
    color: #d3d3d3;
  }

  .primary {
    background: #5b00d1;
    &:hover {
      background: color.adjust(#5b00d1, $lightness: 5%);
    }
    &.disabled {
      background: color.adjust(#5b00d1, $lightness: 20%, $saturation: -20%);
    }
  }

  .secondary {
    background: #005bd1;
    &:hover {
      background: color.adjust(#005bd1, $lightness: 5%);
    }
    &.disabled {
      background: color.adjust(#005bd1, $lightness: 20%, $saturation: -20%);
    }
  }

  .submit {
    background: #28aa01;
    &:hover {
      background: color.adjust(#28aa01, $lightness: 5%);
    }
    &.disabled {
      background: color.adjust(#28aa01, $lightness: 20%, $saturation: -20%);
    }
  }

  .danger {
    background: #aa2301;
    &:hover {
      background: color.adjust(#aa2301, $lightness: 5%);
    }
    &.disabled {
      background: color.adjust(#aa2301, $lightness: 20%, $saturation: -20%);
    }
  }
</style>