<script lang="ts">
	import { getDeviconClass } from '$lib/utils/devicon';
	import { Package, Download } from '@lucide/svelte';

	interface Props {
		name: string;
		variant: 'official' | 'aur';
		isSelected?: boolean;
		onclick?: () => void;
	}

	let { name, variant, isSelected = false, onclick }: Props = $props();

	let devIcon = $derived(getDeviconClass(name));
</script>

{#if variant === 'official'}
	<button
		type="button"
		class="flex items-center gap-2 rounded-lg border px-3 py-2 transition-all duration-200 cursor-pointer
			{isSelected
			? 'border-[#26A768] bg-[#26A768]/10 ring-1 ring-[#26A768]/40 shadow-[0_0_12px_rgba(84,205,76,0.15)]'
			: 'border-border bg-muted/50 hover:bg-muted hover:border-border/80'}"
		{onclick}
	>
		{#if devIcon}
			<i
				class="{devIcon} shrink-0 text-lg {isSelected ? 'text-[#26A768]' : 'text-muted-foreground'}"
			></i>
		{:else}
			<Package class="h-4 w-4 shrink-0 {isSelected ? 'text-[#26A768]' : 'text-muted-foreground'}" />
		{/if}
		<span class="font-mono text-sm {isSelected ? 'text-[#26A768] font-semibold' : ''}">{name}</span>
	</button>
{:else}
	<button
		type="button"
		class="flex items-center gap-2 rounded-lg border border-l-2 px-3 py-2 transition-all duration-200 cursor-pointer
			{isSelected
			? 'border-amber-500/60 border-l-amber-500 bg-amber-500/10 ring-1 ring-amber-500/30 shadow-[0_0_12px_rgba(245,158,11,0.15)]'
			: 'border-border border-l-amber-500/50 bg-card hover:bg-muted/30 hover:border-border/80'}"
		{onclick}
	>
		{#if devIcon}
			<i class="{devIcon} shrink-0 text-lg text-amber-500"></i>
		{:else}
			<Download class="h-4 w-4 shrink-0 text-amber-500" />
		{/if}
		<span class="font-mono text-sm {isSelected ? 'text-amber-400 font-semibold' : ''}">{name}</span>
	</button>
{/if}
