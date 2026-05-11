<script lang="ts">
	import { ModeWatcher } from 'mode-watcher';
	import { QueryClientProvider } from '@tanstack/svelte-query';
	import { commands } from '$lib/commands';
	import './layout.css';
	import 'devicon/devicon.min.css';
	import { onMount, type Snippet } from 'svelte';
	import { queryClient } from '$lib/utils/client-provider';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();

	onMount(async () => {
		await commands.showMainWindow();
	});
</script>

<ModeWatcher defaultMode="system" disableTransitions={false} />
<QueryClientProvider client={queryClient}>
	{@render children?.()}
</QueryClientProvider>
