<script lang="ts">
	import { QueryClientProvider } from '@tanstack/svelte-query';
	import { commands } from '$lib/commands';
	import './layout.css';
	import { onMount, type Snippet } from 'svelte';
	import { queryClientProvider } from '$lib/utils/client-provider';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();

	onMount(async () => {
		await commands.showMainWindow();
	});
</script>

<QueryClientProvider client={queryClientProvider}>
	{@render children?.()}
</QueryClientProvider>
