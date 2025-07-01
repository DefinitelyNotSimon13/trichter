<script lang="ts">
	import '../app.css';
	import { authClient } from '$lib/auth-client';
	import { onMount } from 'svelte';

	async function session() {
		let sess = await authClient.getSession();
		authClient.useSession();
		console.log('Session', sess);
	}

	onMount(async () => {
		await session();
	});

	let { children } = $props();
</script>

<div class="flex h-full w-full flex-col items-center justify-center align-middle">
	{@render children()}
</div>
