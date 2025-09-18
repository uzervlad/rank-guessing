<script lang="ts">
  import type { ApiUser } from "$lib/types/user";

  export let user: ApiUser | null;
</script>

{#if user != null}
{#if user.is_admin}
<a href='/admin'>
  <div class="user">
    <img src={`https://a.ppy.sh/${user.id}`} alt="">
    <span>{user.username}</span>
  </div>
</a>
{:else}
<div class="user">
  <img src={`https://a.ppy.sh/${user.id}`} alt="">
  <span>{user.username}</span>
</div>
{/if}
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