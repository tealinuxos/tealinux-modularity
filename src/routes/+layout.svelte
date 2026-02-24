<script lang="ts">
	import { commands } from '$lib/commands';
	import './layout.css';
	import { onMount, type Snippet } from 'svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();

	onMount(async () => {
		await commands.showMainWindow();
	});
</script>

{@render children?.()}
