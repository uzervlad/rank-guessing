<script lang="ts">
	import { env } from '$env/dynamic/public';
	import favicon from '$lib/assets/favicon.svg';
	import User from '$lib/components/User.svelte';

	let { data, children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<title>{env.PUBLIC_TITLE}</title>
</svelte:head>

<User user={data.user} />

{@render children?.()}

<style lang="scss">
	:global(*) {
		color: white;
		font-family: "Lexend", sans-serif;
		box-sizing: border-box;
	}

	:global(:root) {
		background: #222;
		font-optical-sizing: auto;
	}

	:global(body) {
		margin: 0;
		min-height: 100vh;
		display: flex;
		justify-content: center;
		align-items: center;
	}

	:global(main) {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}
</style>
