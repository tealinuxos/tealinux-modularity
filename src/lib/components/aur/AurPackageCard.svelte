<script lang="ts">
	import {
		Download,
		Trash2,
		Star,
		TrendingUp,
		AlertTriangle,
		Loader2,
		CheckCircle2,
		XCircle,
		ChevronDown
	} from '@lucide/svelte';
	import PackageBadge from '../home/PackageBadge.svelte';
	import ServiceGuideCard from '../home/ServiceGuideCard.svelte';
	import packageGuidesRaw from '$lib/data/packageGuides.json';

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

	let isExpanded = $state(false);

	const packageGuides = packageGuidesRaw as Record<string, any>;
	let guide = $derived(packageGuides[pkg.name] ?? null);

	function formatPopularity(pop: number): string {
		if (pop >= 100) return pop.toFixed(0);
		if (pop >= 10) return pop.toFixed(1);
		return pop.toFixed(2);
	}

	let copiedCmd = $state<string | null>(null);
	let copyTimeout: ReturnType<typeof setTimeout> | null = null;

	async function copyToClipboard(cmd: string) {
		try {
			await navigator.clipboard.writeText(cmd);
		} catch {
			const ta = document.createElement('textarea');
			ta.value = cmd;
			ta.style.position = 'fixed';
			ta.style.opacity = '0';
			document.body.appendChild(ta);
			ta.select();
			document.execCommand('copy');
			document.body.removeChild(ta);
		}
		copiedCmd = cmd;
		if (copyTimeout) clearTimeout(copyTimeout);
		copyTimeout = setTimeout(() => {
			copiedCmd = null;
		}, 1500);
	}

	function toggleExpand(e: MouseEvent) {
		isExpanded = !isExpanded;
		if (onclick) onclick();
	}
</script>

<div
	data-slot="card"
	class="bg-card text-card-foreground flex flex-col gap-6 rounded-xl border group w-full text-left cursor-pointer hover:border-[#26A768]/30 hover:shadow-[0_4px_24px_-8px_rgba(84,205,76,0.15)] transition-all duration-300"
	onclick={toggleExpand}
	onkeydown={(e) => e.key === 'Enter' && (isExpanded = !isExpanded)}
	role="button"
	tabindex="0"
>
	<!-- Header -->
	<div
		data-slot="card-header"
		class="@container/card-header grid auto-rows-min grid-rows-[auto_auto] items-start gap-1.5 px-6 pt-6 has-[[data-slot=card-action]]:grid-cols-[1fr_auto] has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6"
	>
		<!-- Title Area -->
		<div class="flex items-center gap-2 flex-wrap">
			<h4 data-slot="card-title" class="leading-none text-[0.95rem] font-bold text-foreground">
				{pkg.name}
			</h4>
			<PackageBadge
				name={pkg.version}
				variant="aur"
				isSelected={false}
				onclick={(e: any) => e.stopPropagation()}
			/>
			{#if pkg.out_of_date}
				<span
					class="flex items-center gap-1 px-1.5 py-0.5 rounded text-[10px] font-bold uppercase tracking-widest bg-destructive/10 text-destructive border border-destructive/20 shadow-sm"
				>
					<AlertTriangle class="w-3 h-3" /> Outdated
				</span>
			{/if}
		</div>

		<!-- Description Area -->
		<p
			data-slot="card-description"
			class="text-muted-foreground text-[0.8rem] line-clamp-2 leading-relaxed"
		>
			{pkg.description || 'No description available for this package.'}
		</p>

		<!-- Action -->
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<div
			data-slot="card-action"
			class="col-start-2 row-span-2 row-start-1 self-start justify-self-end shrink-0 flex items-center gap-2"
			onclick={(e) => e.stopPropagation()}
			role="group"
			tabindex="-1"
		>
			{#if installState === 'installing'}
				<div
					class="flex items-center gap-1.5 px-3 py-[0.4rem] rounded-lg bg-[#26A768]/10 text-[#26A768] text-[0.72rem] font-bold uppercase tracking-widest"
				>
					<Loader2 class="w-3.5 h-3.5 animate-spin" /> Installing
				</div>
			{:else if installState === 'success'}
				<div
					class="flex items-center gap-1.5 px-3 py-[0.4rem] rounded-lg bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 text-[0.72rem] font-bold uppercase tracking-widest"
				>
					<CheckCircle2 class="w-3.5 h-3.5" /> Done
				</div>
			{:else if installState === 'error'}
				<div
					class="flex items-center gap-1.5 px-3 py-[0.4rem] rounded-lg bg-destructive/10 text-destructive text-[0.72rem] font-bold uppercase tracking-widest"
				>
					<XCircle class="w-3.5 h-3.5" /> Failed
				</div>
			{:else if pkg.installed}
				<button
					onclick={() => onremove?.(pkg.name)}
					class="flex items-center gap-1.5 px-3 py-[0.4rem] rounded-[0.55rem] bg-red-500 border-none text-white hover:bg-red-600 shadow-[0_3px_12px_rgba(239,68,68,0.25)] hover:shadow-[0_4px_16px_rgba(239,68,68,0.35)] hover:-translate-y-px text-[0.75rem] font-extrabold transition-all cursor-pointer break-keep whitespace-nowrap"
				>
					<Trash2 class="w-[0.9rem] h-[0.9rem]" /> Remove
				</button>
			{:else}
				<button
					onclick={() => oninstall?.(pkg.name)}
					class="flex items-center gap-1.5 px-3 py-[0.4rem] rounded-[0.55rem] bg-[#26A768] border-none text-[#052e16] hover:bg-[#4bc043] shadow-[0_3px_12px_rgba(84,205,76,0.3)] hover:shadow-[0_4px_16px_rgba(84,205,76,0.4)] hover:-translate-y-px text-[0.75rem] font-extrabold transition-all cursor-pointer break-keep whitespace-nowrap"
				>
					<Download class="w-[0.9rem] h-[0.9rem]" /> Install
				</button>
			{/if}
		</div>
	</div>

	<!-- Content Area -->
	{#if isExpanded}
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<div
			data-slot="card-content"
			class="px-6 [&:last-child]:pb-6 animate-in slide-in-from-top-1 fade-in duration-200"
			onclick={(e) => e.stopPropagation()}
			role="group"
			tabindex="-1"
		>
			<div class="flex flex-col gap-3">
				<h4
					class="text-[0.65rem] font-extrabold uppercase tracking-[0.15em] text-muted-foreground ml-1"
				>
					Setup & Configuration
				</h4>
				{#if guide}
					<ServiceGuideCard svc={pkg.name} {guide} {copiedCmd} onCopy={copyToClipboard} />
				{:else}
					<div class="flex items-start gap-3 p-4 rounded-xl border border-border/50 bg-muted/20">
						<div
							class="w-8 h-8 rounded-full bg-muted/60 flex items-center justify-center shrink-0 shadow-sm border border-border/40"
						>
							<AlertTriangle class="w-4 h-4 text-muted-foreground" />
						</div>
						<div class="flex flex-col gap-0.5">
							<p class="text-[0.82rem] font-medium text-foreground m-0 leading-tight">
								No specific setup guide available for <strong>{pkg.name}</strong>.
							</p>
							<p class="text-[0.75rem] text-muted-foreground m-0 leading-relaxed">
								If this is a service package, it will be enabled automatically during installation.
								Otherwise, the package is ready for use as soon as installation completes.
							</p>
						</div>
					</div>
				{/if}
			</div>
		</div>
	{/if}

	<!-- Footer -->
	<div
		data-slot="card-footer"
		class="flex flex-wrap items-center px-6 pb-6 [.border-t]:pt-6 justify-between border-t border-border/40 pt-4 gap-y-3 gap-x-4"
	>
		<div
			class="flex flex-wrap items-center gap-2 sm:gap-3.5 text-[0.68rem] font-semibold text-muted-foreground/70 min-w-0"
		>
			<span class="flex items-center gap-1 tooltip-area shrink-0" title="Votes"
				><Star class="w-[0.8rem] h-[0.8rem] text-muted-foreground/50" /> {pkg.num_votes}</span
			>
			<span class="flex items-center gap-1 tooltip-area shrink-0" title="Popularity"
				><TrendingUp class="w-[0.8rem] h-[0.8rem] text-muted-foreground/50" />
				{formatPopularity(pkg.popularity)}</span
			>
			<span class="truncate min-w-0"
				>BY <span class="text-foreground/80 font-bold uppercase tracking-wider"
					>{pkg.maintainer}</span
				></span
			>
			{#if pkg.installed}
				<span
					class="ml-1 flex items-center gap-1 text-[0.62rem] font-extrabold uppercase tracking-widest text-[#26A768]/90 bg-[#26A768]/10 px-1.5 py-0.5 rounded-sm shrink-0"
				>
					<CheckCircle2 class="w-[0.7rem] h-[0.7rem]" /> Installed
				</span>
			{/if}
		</div>
		<div
			class="shrink-0 flex items-center gap-1.5 text-[0.68rem] text-muted-foreground font-semibold uppercase tracking-widest transition-colors group-hover:text-[#26A768]"
		>
			{isExpanded ? 'Hide Details' : 'Details & Setup'}
			<ChevronDown
				class="w-[0.8rem] h-[0.8rem] transition-transform duration-300 {isExpanded
					? 'rotate-180'
					: ''}"
			/>
		</div>
	</div>
</div>
