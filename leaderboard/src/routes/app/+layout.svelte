<script lang="ts">
	import Dock from '$lib/ui/components/dock.svelte';

	let { children } = $props();
</script>

<div class="flex min-h-screen w-full flex-col items-center justify-between align-middle">
	<div>{@render children()}</div>
	<Dock />
</div>
