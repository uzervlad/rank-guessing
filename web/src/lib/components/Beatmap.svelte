<script lang="ts">
  import type { ApiBeatmap } from "$lib/types/beatmap";

  let { beatmap }: { beatmap: ApiBeatmap } = $props();
</script>

<div class="beatmap">
  <img src={`https://assets.ppy.sh/beatmaps/${beatmap.beatmapset_id}/covers/cover.jpg`} alt="beatmap cover" class="cover">
  <div class="info">
    <span class="title">{beatmap.artist} - {beatmap.title}</span>
    <span class="difficulty">[{beatmap.version}] by {beatmap.creator}</span>
  </div>
</div>

<style lang="scss">
  .beatmap {
    background: #333;
    border: 1px solid #888;
    border-radius: 8px;
    overflow: hidden;

    .cover {
      max-width: 100%;
    }

    .info {
      margin: 10px;

      display: flex;
      flex-direction: column;
      gap: 6px;
    }
  }
</style>