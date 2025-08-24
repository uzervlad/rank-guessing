<script lang="ts">
  import type { Payload } from "../../hooks.server";

  export let user: Payload | null;
</script>

{#if user != null}
<div class="user">
  <img src={`https://a.ppy.sh/${user.id}`} alt="">
  <span>{user.username}</span>
</div>
{/if}

<style lang="scss">
  .user {
    background: #5b00d1;

    position: absolute;
    top: 12px;
    right: 12px;

    display: flex;
    align-items: center;
    gap: 6px;

    padding: 6px 9px;
    border-radius: 4px;
  }

  img {
    max-height: 24px;
    border-radius: 50%;
  }
</style>