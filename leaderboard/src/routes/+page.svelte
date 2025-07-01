<script lang="ts">
	import type { SessionResult } from '$lib/models/session_result';
	import { Check } from '@lucide/svelte';
	import { enhance } from '$app/forms';
	import { onDestroy, onMount } from 'svelte';
	import { source } from 'sveltekit-sse';

	let results: SessionResult[] = $state([]);
	let totalResults = $derived(results.length);
	let quickest = $derived.by(() => {
		return results.length
			? results.reduce((min, curr) => (curr.rate < min.rate ? curr : min))
			: null;
	});

	async function fetchInitialResults() {
		const res = await fetch('/api/v1/results');
		results = await res.json();
	}

	async function setupServerSideEvents() {
		source('/api/v1/results/sse')
			.select('new-result')
			.json()
			.subscribe((value: SessionResult) => {
				if (!value) return;
				console.log('NEW');
				results = [...results, value];
			});

		source('/api/v1/results/sse')
			.select('updated-result')
			.json()
			.subscribe((value: SessionResult) => {
				if (!value) return;
				console.log('UPDATED');
				results = results.map((r) => (r.id === value.id ? value : r));
			});
	}

	async function sendDummy() {
		await fetch('/api/v1/results', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({
				rate: Math.random() * 1000,
				duration: Math.floor(Math.random() * 10000)
			})
		});
	}

	function onEnhance() {}

	onMount(() => {
		fetchInitialResults();
		setupServerSideEvents();
	});
</script>

<div class="mt-10 flex flex-col gap-1">
	<h1 class="text-primary text-center text-4xl font-bold">Leaderbord</h1>
	<div class="stats shadow">
		<div class="stat">
			<div class="stat-figure text-primary"></div>
			<div class="stat-title">Total Trichters</div>
			<div class="stat-value text-secondary">{totalResults}</div>
		</div>

		{#if quickest}
			<div class="stat">
				<div class="stat-figure text-secondary"></div>
				<div class="stat-value">{quickest.rate.toFixed(0)} L/min</div>
				<div class="stat-title">That's a lot of beer!</div>
				<div class="stat-desc text-secondary">{quickest.name ?? 'Unknown'} drinks really fast!</div>
			</div>
		{/if}
	</div>
	<div class="divider"></div>

	<div class="rounded-box border-base-content/5 bg-base-100 overflow-x-auto border">
		<table class="table">
			<!-- head -->
			<thead>
				<tr>
					<th></th>
					<th>Name</th>
					<th>Time</th>
					<th>Amount</th>
					<th>Flow Rate</th>
				</tr>
			</thead>
			<tbody>
				{#if results}
					{#each results as result, i (result.id)}
						<tr>
							<th>{i + 1}</th>
							{#if result.name && result.name != 'Unknown'}
								<td>{result.name}</td>
							{:else}
								<td>
									<form
										class="join"
										method="POST"
										action="?/updateName"
										use:enhance={({ formElement, formData, action, cancel, submitter }) => {
											// `formElement` is this `<form>` element
											// `formData` is its `FormData` object that's about to be submitted
											// `action` is the URL to which the form is posted
											// calling `cancel()` will prevent the submission
											// `submitter` is the `HTMLElement` that caused the form to be submitted

											return async ({ result, update }) => {
												// `result` is an `ActionResult` object
												// `update` is a function which triggers the default logic that would be triggered if this callback wasn't set
											};
										}}
									>
										<input type="hidden" name="id" value={result.id} />
										admin()
										<label class="input join-item input-ghost p-0.5 focus:border-0">
											<input type="text" placeholder="Unknown" name="drinker-name" required />
										</label>
										<button class="btn join-item" type="submit">
											<Check />
										</button>
									</form>
								</td>
							{/if}
							<td>{result.duration}s</td>
							<td>{(result.rate * (result.duration / 60)).toFixed(2)}L</td>
							<td>{result.rate.toFixed(2)}L/min</td>
						</tr>
					{/each}
				{/if}
			</tbody>
		</table>
	</div>
	<!-- <button onclick={sendDummy} class="btn">Add Dummy Result</button> -->
</div>
