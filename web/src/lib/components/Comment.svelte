<script lang="ts">
	import type { Emote } from "$lib/emotes";

	let { comment, getEmote }: { comment: string, getEmote: (name: string) => Emote | undefined } = $props();

	const parseComment = (text: string) => {
		const parts = text.split(/(:\w+:)/);

		return parts
			.filter(p => p)
			.map(p => {
				if (p.startsWith(":") && p.endsWith(":")) {
					const emote = getEmote(p.slice(1, -1));
					if (!emote) return { text: p }
					return { text: p, emote: emote.url };
				}

				return { text: p };
			});
	};

	let segments = $derived(parseComment(comment));
</script>

{#each segments as segment}
	{#if segment.emote}
		<img src={segment.emote} alt={segment.text}>
	{:else}
		<span>{segment.text}</span>
	{/if}
{/each}