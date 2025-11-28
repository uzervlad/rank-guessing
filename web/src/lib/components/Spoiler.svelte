<script lang="ts">
	let open = $state(false);

	let symbol = $derived(() => open ? '🔼' : '🔽');

	const { title = '', children } = $props();
</script>

<div class="spoiler">
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<span
		class="header"
		onclick={() => open = !open}
	>{title} {symbol()}</span>
	{#if open}
		<span class="content">{@render children?.()}</span>
	{/if}
</div>

<style lang="scss">
	.spoiler {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.header {
		font-size: 18px;
		font-weight: 600;
		cursor: pointer;
		user-select: none;
	}
	
	.content {
		color: #aaa;
		text-align: center;
	}
</style>