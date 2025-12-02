<script lang="ts">
	import type { Snippet } from 'svelte';
	import './layout.css';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();
</script>

{@render children()}
