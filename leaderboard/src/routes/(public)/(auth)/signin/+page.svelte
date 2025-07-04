<script lang="ts">
	import { goto } from '$app/navigation';
	import { authClient } from '$lib/auth/client';
	import { onMount } from 'svelte';

	let email = $state('');
	let password = $state('');

	let error = $state<string | null>(null);
	let loading = $state(false);

	onMount(async () => {
		const session = await authClient.getSession();
		if (session.data?.user) {
			goto('/app');
		}
	});
	async function handleSubmit(e: Event) {
		e.preventDefault();
		error = null;
		loading = true;

		const { error: authError } = await authClient.signIn.email({
			email,
			password
		});

		if (authError) {
			error = authError.message ?? null;
			loading = false;
			return;
		}

		goto('/app');
	}
</script>

<div class="flex h-screen flex-col justify-center align-middle">
	<div class="mockup-window border-base-300 border">
		<form class="p-10 pt-5" onsubmit={handleSubmit}>
			<fieldset class="fieldset">
				<legend class="fieldset-legend">Email</legend>
				<input
					type="email"
					class="input validator"
					placeholder="Type here"
					name="email"
					bind:value={email}
				/>
			</fieldset>
			<fieldset class="fieldset">
				<legend class="fieldset-legend">Password</legend>
				<input
					type="password"
					class="input validator"
					placeholder="Type here"
					name="password"
					bind:value={password}
				/>
			</fieldset>
			{#if error}
				<p class="text-error text-s mb-4">{error}</p>
			{/if}

			<div class="divider"></div>
			<div class="mb-0 flex flex-col justify-center">
				<button class="btn btn-primary w-full" type="submit" disabled={loading}>
					{#if loading}
						Signing in…
					{:else}
						Sign In
					{/if}
				</button>
				<a class="text-info text-s text-center underline" href="/signup">SignUp instead</a>
			</div>
		</form>
	</div>
</div>
