<script lang="ts">
	import { Toaster } from 'svelte-sonner';
	import { ModeWatcher } from 'mode-watcher';
	import { QueryClientProvider } from '@tanstack/svelte-query';
	import { commands } from '$lib/commands';
	import './layout.css';
	import 'devicon/devicon.min.css';
	import { onMount, onDestroy, type Snippet } from 'svelte';
	import { queryClient } from '$lib/utils/client-provider';
	import InstallOverlay from '$lib/components/InstallOverlay.svelte';
	import { initInstallListeners, cleanupInstallListeners } from '$lib/services/installManager';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();

	onMount(async () => {
		await commands.showMainWindow();
		// Register global install event listeners once — they persist across navigation
		await initInstallListeners();
	});

	onDestroy(() => {
		// Only called when the entire app tears down, not on navigation
		cleanupInstallListeners();
	});
</script>

<Toaster richColors />
<ModeWatcher defaultMode="system" disableTransitions={false} />

<!-- Global install overlay — renders above everything, persists during navigation -->
<InstallOverlay />

<QueryClientProvider client={queryClient}>
	{@render children?.()}
</QueryClientProvider>
