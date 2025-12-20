<script lang="ts">
	import type { Snippet } from 'svelte';
	import './layout.css';
	import TeaBar from '$lib/components/TeaBar.svelte';
	import * as Sidebar from '$lib/components/ui/sidebar/index';
	import { ModeWatcher } from 'mode-watcher';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();
</script>

<ModeWatcher defaultMode="system" />
<Sidebar.Provider>
	<TeaBar />
	<main class="flex flex-col flex-1 w-full">
		<Sidebar.Trigger />
		<section class="p-4">
			{@render children()}
		</section>
	</main>
</Sidebar.Provider>
