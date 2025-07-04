<script lang="ts">
	import { Home, Inbox, Settings } from '@lucide/svelte';
</script>

<div class="flex w-screen justify-center">
	<div class="dock relative w-fit">
		<button class="mt-5 mr-7 mb-5 ml-7">
			<Home />
			<span class="dock-label">Home</span>
		</button>

		<button class="mt-5 mr-7 mb-5 ml-7">
			<Inbox />
			<span class="dock-label">Inbox</span>
		</button>

		<button class="mt-5 mr-7 mb-5 ml-7">
			<Settings />
			<span class="dock-label">Settings</span>
		</button>
	</div>
</div>
