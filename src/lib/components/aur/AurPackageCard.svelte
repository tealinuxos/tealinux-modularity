<script lang="ts">
	import {
		Download,
		Trash2,
		Star,
		TrendingUp,
		AlertTriangle,
		Loader2,
		CheckCircle2,
		XCircle
	} from '@lucide/svelte';

	interface AurPackage {
		name: string;
		version: string;
		description: string;
		maintainer: string;
		num_votes: number;
		popularity: number;
		out_of_date: boolean;
		installed: boolean;
	}

	interface Props {
		pkg: AurPackage;
		installState?: 'idle' | 'installing' | 'success' | 'error';
		oninstall?: (name: string) => void;
		onremove?: (name: string) => void;
		onclick?: () => void;
	}

	let { pkg, installState = 'idle', oninstall, onremove, onclick }: Props = $props();

	function formatPopularity(pop: number): string {
		if (pop >= 100) return pop.toFixed(0);
		if (pop >= 10) return pop.toFixed(1);
		return pop.toFixed(2);
	}
</script>

<button
	id="aur-card-{pkg.name}"
	{onclick}
	class="group w-full text-left rounded-xl border border-border bg-card p-4
           hover:border-primary/30 hover:shadow-md
           transition-all duration-200 cursor-pointer"
>
	<div class="flex items-start justify-between gap-3 mb-2">
		<div class="flex items-center gap-2 min-w-0">
			<h3 class="text-sm font-semibold text-foreground truncate">{pkg.name}</h3>
			<span
				class="shrink-0 px-1.5 py-0.5 rounded text-[10px] font-mono font-medium bg-primary/10 text-primary"
			>
				{pkg.version}
			</span>
			{#if pkg.out_of_date}
				<span
					class="shrink-0 flex items-center gap-1 px-1.5 py-0.5 rounded text-[10px] font-medium bg-destructive/10 text-destructive"
				>
					<AlertTriangle class="w-3 h-3" />
					outdated
				</span>
			{/if}
		</div>

		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<div class="shrink-0" onclick={(e) => e.stopPropagation()} role="group">
			{#if installState === 'installing'}
				<div
					class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-primary/10 text-primary text-xs font-medium"
				>
					<Loader2 class="w-3.5 h-3.5 animate-spin" />
					Installing...
				</div>
			{:else if installState === 'success'}
				<div
					class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-green-500/10 text-green-600 dark:text-green-400 text-xs font-medium"
				>
					<CheckCircle2 class="w-3.5 h-3.5" />
					Done
				</div>
			{:else if installState === 'error'}
				<div
					class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-destructive/10 text-destructive text-xs font-medium"
				>
					<XCircle class="w-3.5 h-3.5" />
					Failed
				</div>
			{:else if pkg.installed}
				<button
					onclick={() => onremove?.(pkg.name)}
					class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-destructive/10 text-destructive hover:bg-destructive/20 text-xs font-medium transition-colors"
				>
					<Trash2 class="w-3.5 h-3.5" />
					Remove
				</button>
			{:else}
				<button
					onclick={() => oninstall?.(pkg.name)}
					class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-primary/10 text-primary hover:bg-primary/20 text-xs font-medium transition-colors"
				>
					<Download class="w-3.5 h-3.5" />
					Install
				</button>
			{/if}
		</div>
	</div>

	<p class="text-xs text-muted-foreground line-clamp-2 mb-3 leading-relaxed">
		{pkg.description || 'No description available'}
	</p>

	<div class="flex items-center gap-4 text-[11px] text-muted-foreground">
		<span class="flex items-center gap-1"><Star class="w-3.5 h-3.5" />{pkg.num_votes}</span>
		<span class="flex items-center gap-1"
			><TrendingUp class="w-3.5 h-3.5" />{formatPopularity(pkg.popularity)}</span
		>
		<span class="truncate">by {pkg.maintainer}</span>
		{#if pkg.installed}
			<span
				class="ml-auto shrink-0 flex items-center gap-1 px-1.5 py-0.5 rounded bg-primary/10 text-primary font-medium"
			>
				<CheckCircle2 class="w-3 h-3" />
				installed
			</span>
		{/if}
	</div>
</button>
