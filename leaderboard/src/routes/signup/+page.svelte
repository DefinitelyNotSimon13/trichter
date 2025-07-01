<script lang="ts">
	import type { PageProps } from '../$types';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	let { data, form }: PageProps = $props();

	console.log('Form Success:', form?.success);

	onMount(() => {
		if (form && form?.success === true) {
			goto('/');
		} else if (form && form?.success === false) {
			alert('Failed to login');
		}
	});
</script>

<div class="flex h-screen flex-col justify-center align-middle">
	<div class="mockup-window border-base-300 border">
		<form class="p-10 pt-5" method="POST">
			<fieldset class="fieldset">
				<legend class="fieldset-legend">What is your name?</legend>
				<input type="text" class="input" placeholder="Type here" name="name" />
			</fieldset>
			<fieldset class="fieldset">
				<legend class="fieldset-legend">What is your email?</legend>
				<input type="email" class="input validator" placeholder="Type here" name="email" />
			</fieldset>
			<fieldset class="fieldset">
				<legend class="fieldset-legend">Password</legend>
				<input type="password" class="input validator" placeholder="Type here" name="password" />
			</fieldset>

			<div class="divider"></div>
			<button class="btn btn-primary w-full" type="submit">SignUp</button>
		</form>
	</div>
</div>
