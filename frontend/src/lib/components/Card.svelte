<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		title: string;
		children?: Snippet;
	}

	let { title, children }: Props = $props();
</script>

<div class="rounded-lg bg-white shadow-md">
	<div class="border-b border-gray-200 px-6 py-4">
		<h2 class="text-lg font-medium">{title}</h2>
	</div>
	<div class="p-6">
		{@render children?.()}
	</div>
</div>
