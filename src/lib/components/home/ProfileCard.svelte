<script lang="ts">
	import { Download, Package, ChevronRight, Server, LoaderCircle, Check, X } from '@lucide/svelte';
	import { getCategoryIcon, getCategoryColor, getCategoryBadgeColor } from '$lib/utils/category';
	import { Button } from '$lib/components/ui/button';

	interface Props {
		id: string;
		title: string;
		description: string;
		category: string;
		packageCount: number;
		packages: string[];
		servicesCount: number;
		onInstall: () => void;

		installState: 'idle' | 'installing' | 'success' | 'error';
	}

	let {
		id,
		title,
		description,
		category,
		packageCount,
		packages,
		servicesCount,
		onInstall,

		installState = 'idle'
	}: Props = $props();

	let IconComponent = $derived(getCategoryIcon(category));
	let tintClass = $derived(getCategoryColor(category));
	let badgeClass = $derived(getCategoryBadgeColor(category));

	let useFallback = $state(false);
	let iconPath = $derived(`/icons/${id}.svg`);
</script>

<div
	class="surface surface-outline surface-hover group relative overflow-hidden rounded-[var(--radius-xl)] border border-border/60"
>
	<div class="p-6 md:p-7">
		<!-- Top Row: Icon + Category Badge -->
		<div class="flex items-start justify-between gap-4 mb-4">
			<div
				class="flex h-12 w-12 items-center justify-center rounded-[var(--radius-lg)] border border-border/60 bg-muted/35 backdrop-blur-sm {tintClass}"
			>
				{#if !useFallback}
					<img
						src={iconPath}
						alt={title}
						class="h-9 w-9 object-contain transition-transform group-hover:scale-110 duration-300"
						onerror={() => (useFallback = true)}
					/>
				{:else}
					<IconComponent class="h-6 w-6 text-foreground/80" />
				{/if}
			</div>
			<span
				class="shrink-0 rounded-full border px-2.5 py-1 text-[0.65rem] font-semibold uppercase tracking-wider {badgeClass}"
			>
				{category}
			</span>
		</div>

		<!-- Title & Description -->
		<h3
			class="mb-1.5 text-[1.05rem] font-semibold tracking-tight text-foreground transition-colors group-hover:text-[color:var(--accent-green)]"
		>
			{title}
		</h3>
		<p class="mb-4 text-sm leading-relaxed text-muted-foreground line-clamp-2">
			{description}
		</p>

		<!-- Stats Row -->
		<div class="mb-5 flex flex-wrap items-center gap-x-4 gap-y-2 text-[0.72rem] font-medium text-muted-foreground">
			<div class="flex items-center gap-1.5">
				<Package class="h-3.5 w-3.5 text-foreground/60" />
				<span>{packageCount} packages</span>
			</div>
			{#if servicesCount > 0}
				<div class="flex items-center gap-1.5">
					<Server class="h-3.5 w-3.5 text-foreground/60" />
					<span>{servicesCount} services</span>
				</div>
			{/if}
		</div>

		<!-- Package w Pills -->
		<div class="flex flex-wrap gap-1.5 mb-5">
			{#each packages.slice(0, 3) as pkg}
				<span
					class="rounded-md border border-border/60 bg-muted/35 px-2 py-0.5 font-mono text-[10.5px] text-muted-foreground"
				>
					{pkg}
				</span>
			{/each}
			{#if packages.length > 3}
				<span
					class="rounded-md border border-border/60 bg-muted/35 px-2 py-0.5 font-mono text-[10.5px] text-muted-foreground"
				>
					+{packages.length - 3} more
				</span>
			{/if}
		</div>

		<!-- Action Buttons -->
		<div class="grid grid-cols-2 gap-3">
			<Button
				onclick={onInstall}
				disabled={installState === 'installing' || installState === 'success'}
				class="h-10 gap-2 rounded-[var(--radius-lg)] px-4 text-sm font-semibold transition-all duration-200
					{installState === 'success'
					? 'bg-green-500/12 text-green-400 border border-green-500/20 cursor-default'
					: installState === 'error'
						? 'bg-red-500/12 text-red-400 border border-red-500/20'
						: installState === 'installing'
							? 'bg-[color:var(--accent-green)]/12 text-[color:var(--accent-green)] border border-[color:var(--accent-green)]/25 cursor-wait'
							: 'bg-[color:var(--accent-green)] text-[#052e16] hover:bg-[color:var(--accent-green-2)] shadow-[0_10px_28px_-18px_color-mix(in_oklab,var(--accent-green)_55%,transparent)] active:scale-[0.98]'}"
			>
				{#if installState === 'installing'}
					<LoaderCircle class="w-4 h-4 animate-spin" />
					Installing...
				{:else if installState === 'success'}
					<Check class="w-4 h-4" />
					Installed
				{:else if installState === 'error'}
					<X class="w-4 h-4" />
					Failed
				{:else}
					<Download class="w-4 h-4" />
					Install
				{/if}
			</Button>
			<Button
				href="/home/{id}"
				variant="outline"
				class="h-10 gap-2 rounded-[var(--radius-lg)] border-border/70 bg-transparent px-4 text-sm font-medium hover:bg-muted/40"
			>
				Details
				<ChevronRight class="w-4 h-4" />
			</Button>
		</div>
	</div>
</div>
