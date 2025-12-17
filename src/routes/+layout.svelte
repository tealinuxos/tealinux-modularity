<script lang="ts">
	import type { Snippet } from 'svelte';
	import './layout.css';
	import TeaBar from '$lib/components/TeaBar.svelte';
	import * as Sidebar from '$lib/components/ui/sidebar/index';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();
</script>

<Sidebar.Provider>
	<TeaBar />
	<main class="flex flex-col flex-1 w-full">
		<Sidebar.Trigger />
		<section class="p-4">
			{@render children()}
		</section>
	</main>
</Sidebar.Provider>
