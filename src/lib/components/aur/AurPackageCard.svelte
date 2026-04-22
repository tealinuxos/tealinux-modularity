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
		ExternalLink
	} from '@lucide/svelte';
	import PackageBadge from '../home/PackageBadge.svelte';
	import AurPackageDetail from './AurPackageDetail.svelte';
	import { commands } from '$lib/commands';
	import { onDestroy } from 'svelte';

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
		installState?: 'idle' | 'installing' | 'uninstalling' | 'success' | 'error';
		oninstall?: (name: string) => void;
		onremove?: (name: string) => void;
	}

	let { pkg, installState = 'idle', oninstall, onremove }: Props = $props();

	function formatPopularity(pop: number): string {
		if (pop >= 100) return pop.toFixed(0);
		if (pop >= 10) return pop.toFixed(1);
		return pop.toFixed(2);
	}

	// Self-contained detail modal
	let detailPkg: any = $state(null);
	let detailOpen = $state(false);

	async function handleCardClick() {
		const info = await commands.getAurPackageInfo(pkg.name);
		if (info) {
			detailPkg = info;
			detailOpen = true;
		}
	}

	// Simulated progress state
	let progress = $state(0);
	let progressInterval: ReturnType<typeof setInterval> | null = null;
	let showProgress = $state(false);

	function startSimulatedProgress() {
		progress = 0;
		showProgress = true;
		if (progressInterval) clearInterval(progressInterval);

		progressInterval = setInterval(() => {
			if (progress < 30) {
				// Fast start: 2-5% increments
				progress += Math.random() * 3 + 2;
			} else if (progress < 60) {
				// Medium: 1-3% increments
				progress += Math.random() * 2 + 1;
			} else if (progress < 85) {
				// Slow: 0.5-1.5% increments
				progress += Math.random() * 1 + 0.5;
			} else if (progress < 95) {
				// Very slow: 0.1-0.5% increments
				progress += Math.random() * 0.4 + 0.1;
			}
			// Cap at 95% until real completion
			progress = Math.min(progress, 95);
		}, 300);
	}

	function stopProgress(final: 'success' | 'error') {
		if (progressInterval) {
			clearInterval(progressInterval);
			progressInterval = null;
		}
		if (final === 'success') {
			progress = 100;
			setTimeout(() => {
				showProgress = false;
				progress = 0;
			}, 600);
		} else {
			showProgress = false;
			progress = 0;
		}
	}

	$effect(() => {
		if (installState === 'installing' || installState === 'uninstalling') {
			startSimulatedProgress();
		} else if (installState === 'success') {
			stopProgress('success');
		} else if (installState === 'error') {
			stopProgress('error');
		} else {
			// idle
			if (progressInterval) {
				clearInterval(progressInterval);
				progressInterval = null;
			}
			showProgress = false;
			progress = 0;
		}
	});

	onDestroy(() => {
		if (progressInterval) clearInterval(progressInterval);
	});
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
	data-slot="card"
	class="bg-card text-card-foreground flex flex-col rounded-xl border group w-full text-left cursor-pointer hover:border-[#26A768]/30 hover:shadow-[0_4px_24px_-8px_rgba(84,205,76,0.15)] transition-all duration-300"
	onclick={handleCardClick}
	onkeydown={(e) => e.key === 'Enter' && handleCardClick()}
	role="button"
	tabindex="0"
>
	<!-- Header -->
	<div
		data-slot="card-header"
		class="@container/card-header grid auto-rows-min grid-rows-[auto_auto] items-start gap-1.5 px-6 pt-6 has-[[data-slot=card-action]]:grid-cols-[1fr_auto] has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6"
	>
		<!-- Title Area -->
		<div class="flex items-center gap-2 flex-wrap min-w-0">
			<h4
				data-slot="card-title"
				class="leading-none text-[0.95rem] font-bold text-foreground truncate"
			>
				{pkg.name}
			</h4>
			<span class="text-[0.7rem] font-mono text-muted-foreground/50 shrink-0">
				{pkg.version}
			</span>
			{#if pkg.out_of_date}
				<span
					class="flex items-center gap-1 px-1.5 py-0.5 rounded text-[9px] font-bold uppercase tracking-wider bg-red-500/10 text-red-500 border border-red-500/20 shadow-sm"
				>
					<AlertTriangle class="w-2.5 h-2.5" /> Outdated
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
			{:else if installState === 'uninstalling'}
				<div
					class="flex items-center gap-1.5 px-3 py-[0.4rem] rounded-lg bg-red-500/10 text-red-500 text-[0.72rem] font-bold uppercase tracking-widest"
				>
					<Loader2 class="w-3.5 h-3.5 animate-spin" /> Uninstalling
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

	<!-- Progress Bar with Percentage -->
	{#if showProgress}
		<div class="progress-section" class:progress-uninstalling={installState === 'uninstalling'}>
			<div class="progress-track">
				<div class="progress-fill" style="width: {progress}%"></div>
			</div>
			<span class="progress-pct">{Math.round(progress)}%</span>
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
				<div class="flex items-center gap-1.5 ml-1 shrink-0">
					<div class="w-1.5 h-1.5 rounded-full bg-[#26A768]"></div>
					<span class="text-[0.65rem] font-bold uppercase tracking-widest text-muted-foreground/60"
						>Synced</span
					>
				</div>
			{/if}
		</div>
		<div
			class="shrink-0 flex items-center gap-1.5 text-[0.68rem] text-muted-foreground font-semibold uppercase tracking-widest transition-colors group-hover:text-[#26A768]"
		>
			View Details
			<ExternalLink class="w-[0.8rem] h-[0.8rem]" />
		</div>
	</div>
</div>

{#if detailOpen}
	<AurPackageDetail
		pkg={detailPkg}
		open={detailOpen}
		{installState}
		onclose={() => {
			detailOpen = false;
			detailPkg = null;
		}}
		oninstall={oninstall ? (name) => oninstall(name) : undefined}
		onremove={onremove ? (name) => onremove(name) : undefined}
	/>
{/if}

<style>
	.progress-section {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0 1.5rem;
		padding-bottom: 0.25rem;
	}

	.progress-track {
		flex: 1;
		height: 3px;
		background: rgba(38, 167, 104, 0.12);
		overflow: hidden;
		border-radius: 999px;
	}

	.progress-uninstalling .progress-track {
		background: rgba(239, 68, 68, 0.12);
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #26a768, #4bc043);
		border-radius: 999px;
		transition: width 0.3s ease-in-out;
	}

	.progress-uninstalling .progress-fill {
		background: linear-gradient(90deg, #ef4444, #f87171);
	}

	.progress-pct {
		font-size: 0.68rem;
		font-weight: 800;
		color: #26a768;
		min-width: 2.2rem;
		text-align: right;
		font-variant-numeric: tabular-nums;
	}

	.progress-uninstalling .progress-pct {
		color: #ef4444;
	}
</style>
