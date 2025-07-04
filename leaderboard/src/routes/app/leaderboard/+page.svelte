<script lang="ts">
	import { Check } from '@lucide/svelte';
	import { enhance } from '$app/forms';
	import { onMount } from 'svelte';
	import { source } from 'sveltekit-sse';
	import type { Run } from '$lib/models/run';
	import { ServerEvent } from '$lib/models/events';

	let runs: Run[] = $state([]);
	let totalRuns = $derived(runs.length);
	let quickest = $derived.by(() => {
		return runs.length ? runs.reduce((max, curr) => (curr.rate < max.rate ? curr : max)) : null;
	});

	// let props: PageProps = $props();

	async function setupServerSideEvents() {
		source('/api/v1/runs/sse')
			.select(ServerEvent.RunCreated)
			.json()
			.subscribe((value: Run) => {
				if (!value) return;
				runs = [...runs, value];
			});

		source('/api/v1/runs/sse')
			.select(ServerEvent.RunUpdated)
			.json()
			.subscribe((value: Run) => {
				if (!value) return;
				runs = runs.map((r) => (r.id === value.id ? value : r));
			});
	}

	onMount(() => {
		setupServerSideEvents();
	});
</script>

<div class="mt-10 flex flex-col gap-1">
	<h1 class="text-primary text-center text-4xl font-bold">Leaderbord</h1>
	<div class="stats shadow">
		<div class="stat">
			<div class="stat-figure text-primary"></div>
			<div class="stat-title">Total Trichters</div>
			<div class="stat-value text-secondary">{totalRuns}</div>
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
				{#if runs}
					{#each runs as run, i (run.id)}
						<tr>
							<th>{i + 1}</th>
							{#if run.name && run.name != 'Unknown'}
								<td>{run.name}</td>
							{:else}
								<td>
									<form class="join" method="POST" action="?/updateName" use:enhance>
										<input type="hidden" name="id" value={run.id} />
										<label class="input join-item input-ghost p-0.5 focus:border-0">
											<input type="text" placeholder="Unknown" name="run-name" required />
										</label>
										<button class="btn join-item" type="submit">
											<Check />
										</button>
									</form>
								</td>
							{/if}
							<td>{run.duration}s</td>
							<td>{run.volume.toFixed(2)}L</td>
							<td>{run.rate.toFixed(2)}L/min</td>
						</tr>
					{/each}
				{/if}
			</tbody>
		</table>
	</div>
	<a class="text-accent mt-5 text-center text-xs underline" href="/signout">Sign Out</a>
</div>
