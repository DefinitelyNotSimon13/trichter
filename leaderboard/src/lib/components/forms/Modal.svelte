<script>
	export let open = false;
	export let onClose = () => {};
	export let size = 'md';

	const sizeClasses = {
		sm: 'max-w-sm',
		md: 'max-w-md',
		lg: 'max-w-lg',
		xl: 'max-w-xl',
		'2xl': 'max-w-2xl',
		'3xl': 'max-w-3xl',
		'4xl': 'max-w-4xl',
		'5xl': 'max-w-5xl',
		'6xl': 'max-w-6xl',
		'7xl': 'max-w-7xl'
	};

	function handleClose() {
		open = false;
		onClose();
	}
</script>

<dialog class="modal" class:modal-open={open}>
	<div class="modal-box w-11/12 {sizeClasses[size]}">
		<form method="dialog">
			<button
				type="button"
				class="btn btn-sm btn-circle btn-ghost absolute top-2 right-2"
				onclick={handleClose}
			>
				✕
			</button>
		</form>
		<slot />
	</div>
</dialog>