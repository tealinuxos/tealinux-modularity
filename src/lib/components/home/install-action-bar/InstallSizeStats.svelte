<script lang="ts">
	import { Loader2, Wifi, HardDrive } from '@lucide/svelte';
	import SizeStat from './SizeStat.svelte';

	interface Props {
		selectedCount: number;
		uninstallCount: number;
		totalSize: string;
		totalInstallSize?: string;
		sizeLoading?: boolean;
		estTime: string;
	}

	let {
		selectedCount,
		uninstallCount,
		totalSize,
		totalInstallSize = '',
		sizeLoading = false,
		estTime
	}: Props = $props();

	let hasSizeData = $derived(
		!sizeLoading && totalSize !== '' && totalSize !== '—' && totalSize !== 'Unknown'
	);

	/** Readable package selection summary text */
	let packageLabel = $derived(
		selectedCount > 0 && uninstallCount > 0
			? `${selectedCount} install · ${uninstallCount} remove`
			: selectedCount > 0
				? `${selectedCount} to install`
				: uninstallCount > 0
					? `${uninstallCount} to remove`
					: 'None selected'
	);
</script>

<div class="flex items-center gap-5 min-w-0">
	<!-- Download size -->
	<div class="flex flex-col gap-0.5">
		<p
			class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5"
		>
			<Wifi class="w-3 h-3" />
			Download
		</p>
		{#if sizeLoading}
			<div class="flex items-center gap-1.5">
				<Loader2 class="w-3.5 h-3.5 text-primary animate-spin" />
				<span class="text-sm text-muted-foreground italic">Fetching…</span>
			</div>
		{:else}
			<span
				class={`text-sm font-medium leading-none tabular-nums ${hasSizeData ? 'text-foreground' : 'text-muted-foreground'}`}
			>
				{totalSize || '—'}
			</span>
		{/if}
	</div>

	<!-- On Disk size (only when we have it) -->
	{#if hasSizeData && totalInstallSize}
		<div class="w-px h-9 bg-border flex-shrink-0"></div>
		<SizeStat icon={HardDrive} label="On Disk" value={totalInstallSize} />
	{/if}

	<!-- Divider before package count -->
	<div class="w-px h-9 bg-border flex-shrink-0"></div>

	<!-- Package count -->
	<div class="flex flex-col gap-0.5">
		<p class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">Packages</p>
		<div class="flex items-baseline gap-1.5 flex-wrap">
			{#if selectedCount > 0}
				<span class="text-sm font-normal text-foreground">{selectedCount} to install</span>
			{/if}
			{#if uninstallCount > 0}
				<span class="text-sm font-normal text-red-400">{uninstallCount} to remove</span>
			{/if}
			{#if selectedCount === 0 && uninstallCount === 0}
				<span class="text-sm text-muted-foreground">None selected</span>
			{/if}
		</div>
	</div>


</div>
