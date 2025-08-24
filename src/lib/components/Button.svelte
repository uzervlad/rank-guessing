<script lang="ts">
  type ButtonVariant = 'primary' | 'submit' | 'danger';

  let { children, variant = 'primary', ...props } = $props();
</script>

<button class={variant} {...props}>
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

  .primary {
    background: #5b00d1;
    &:hover {
      background: color.adjust(#5b00d1, $lightness: 5%);
    }
  }

  .secondary {
    background: #005bd1;
    &:hover {
      background: color.adjust(#005bd1  , $lightness: 5%);
    }
  }

  .submit {
    background: #28aa01;
    &:hover {
      background: color.adjust(#28aa01, $lightness: 5%);
    }
  }

  .danger {
    background: #aa2301;
    &:hover {
      background: color.adjust(#aa2301, $lightness: 5%);
    }
  }
</style>