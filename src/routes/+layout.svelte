<script lang="ts">
	import { commands } from '$lib/commands';
	import './layout.css';
	import { onMount, type Snippet } from 'svelte';
	import TeaBar from '$lib/components/TeaBar.svelte';
	import * as Sidebar from '$lib/components/ui/sidebar/index';
	import { ModeWatcher } from 'mode-watcher';
	import { page } from '$app/state';
	import { fade } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();

	onMount(async () => {
		await commands.showMainWindow();
	});
</script>

<ModeWatcher defaultMode="system" />
<Sidebar.Provider>
	<TeaBar />
	<main class="flex flex-col flex-1 w-full">
		<Sidebar.Trigger />
		{#key page.url.pathname}
			<section in:fade={{ duration: 300, easing: cubicOut }} class="p-4 h-full">
				{@render children?.()}
			</section>
		{/key}
	</main>
</Sidebar.Provider>
