<script lang="ts">
	import { goto } from '$app/navigation';
	import { authClient } from '$lib/auth/client';
	import { onMount } from 'svelte';

	onMount(async () => {
		const { data } = await authClient.getSession();
		if (!data?.user) {
			goto('/');
		}

		await authClient.signOut();
		goto('/');
	});
</script>
