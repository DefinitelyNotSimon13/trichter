<script lang="ts">
	import { onMount } from 'svelte';

	let deferredPrompt: any;
	let showInstallButton = false;

	function handleBeforeInstallPrompt(event: Event) {
		// Prevent the mini-infobar from appearing on mobile
		event.preventDefault();
		// Stash the event so it can be triggered later
		deferredPrompt = event;
		// Update UI notify the user they can install the PWA
		showInstallButton = true;
	}

	async function installApp() {
		if (!deferredPrompt) return;
		
		// Show the install prompt
		deferredPrompt.prompt();
		
		// Wait for the user to respond to the prompt
		const { outcome } = await deferredPrompt.userChoice;
		
		// We no longer need the prompt. Clear it up.
		deferredPrompt = null;
		showInstallButton = false;
	}

	onMount(() => {
		// Listen for the beforeinstallprompt event
		window.addEventListener('beforeinstallprompt', handleBeforeInstallPrompt);
		
		// Check if the app is already installed
		if (window.matchMedia('(display-mode: standalone)').matches) {
			showInstallButton = false;
		}

		// Cleanup event listener on component destroy
		return () => {
			window.removeEventListener('beforeinstallprompt', handleBeforeInstallPrompt);
		};
	});
</script>

{#if showInstallButton}
	<button class="btn btn-primary" on:click={installApp}>
		Install App
	</button>
{/if}