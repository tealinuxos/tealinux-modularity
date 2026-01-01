<script lang="ts">
	import './layout.css';
	import type { Snippet } from 'svelte';
	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();
</script>

{@render children?.()}
