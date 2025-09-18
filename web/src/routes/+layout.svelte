<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';
	import User from '$lib/components/User.svelte';

	let { data, children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<title>Koi's Rank Guessing</title>
</svelte:head>

<User user={data.user} />

<main>
	<h1>Koi's Rank Guessing</h1>
	{@render children?.()}
</main>

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
		width: 100%;
		padding: 0 20px;
	}

	h1 {
		text-align: center;
	}
</style>
