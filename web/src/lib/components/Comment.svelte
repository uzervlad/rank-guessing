<script lang="ts">
	import type { Emote } from "$lib/emotes";

	let { comment, getEmote }: { comment: string, getEmote: (name: string) => Emote | undefined } = $props();

	const parseComment = (text: string) => {
		const words = text.split(' ');

		return words
			.filter(p => p)
			.map(p => {
				const emote = getEmote(p);
				if (!emote) return { text: p }
				return { text: p, emote: emote.url, zero: emote.zero_width };
			});
	};

	let segments = $derived(parseComment(comment));
</script>

{#each segments as segment}
	{#if segment.emote}
		<img class="emote" src={segment.emote} alt={segment.text}>{" "}
	{:else}
		<span>{segment.text}{" "}</span>
	{/if}
{/each}

<style lang="scss">
	.emote {
		max-height: 32px;
	}
</style>
