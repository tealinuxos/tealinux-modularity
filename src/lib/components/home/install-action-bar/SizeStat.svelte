<script lang="ts">
	import type { Component } from 'svelte';

	interface Props {
		label: string;
		/** Lucide icon component */
		icon?: Component<{ class?: string }>;
		value: string;
		/** Smaller secondary value shown beside the main one */
		subValue?: string;
		/** Use large bold style for the main value */
		large?: boolean;
		/** Dim the value text (e.g. when no data) */
		muted?: boolean;
	}

	let { label, icon: Icon, value, subValue = '', large = false, muted = false }: Props = $props();
</script>

<div class="flex flex-col gap-0.5">
	<p
		class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5"
	>
		{#if Icon}
			<Icon class="w-3 h-3" />
		{/if}
		{label}
	</p>
	<div class="flex items-baseline gap-1">
		<span
			class={`leading-none tabular-nums font-medium
				${large ? 'text-xl' : 'text-base'}
				${muted ? 'text-muted-foreground' : 'text-foreground'}`}
		>
			{value}
		</span>
		{#if subValue}
			<span class="text-[10px] text-muted-foreground">{subValue}</span>
		{/if}
	</div>
</div>
