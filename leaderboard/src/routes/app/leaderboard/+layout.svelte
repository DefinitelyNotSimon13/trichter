<script lang="ts">
	let { children } = $props();
</script>

<div class="flex w-full flex-col items-center justify-between align-middle">
	<div>{@render children()}</div>
</div>
