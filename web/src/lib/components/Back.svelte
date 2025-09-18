<script lang="ts">
  import { goto } from "$app/navigation";
  import ArrowBigLeftDash from "@lucide/svelte/icons/arrow-big-left-dash";

  const goBack = () => {
    goto(to);
  };

  const { to } = $props();
</script>


<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div class="back" onclick={goBack}>
  <ArrowBigLeftDash color="white" />
</div>

<style lang="scss">
  .back {
    position: absolute;
    top: 12px;
    left: 12px;

    cursor: pointer;
  }
</style>