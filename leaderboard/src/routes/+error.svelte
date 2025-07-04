<script lang="ts">
	import { page } from '$app/state';
	import { logger } from '$lib/logger';
	logger.error(page.error);
</script>

<div class="flex h-screen w-full flex-col items-center justify-center">
	{#if page.error}
		{#if page.error.code}
			<h1 class="text-error text-5xl font-bold">{page.error.code}</h1>
			<h2 class="text-error-content text-2xl">{page.error.message}</h2>
		{:else}
			<h2 class="text-error text-2xl font-bold">{page.error.message}</h2>
		{/if}
	{:else}
		<h1 class="text-error text-3xl font-bold">Unexpteced Error</h1>
	{/if}
</div>
