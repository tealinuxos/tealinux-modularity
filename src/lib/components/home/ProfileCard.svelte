<script lang="ts">
	import {
		Download,
		Package,
		ChevronRight,
		Shield,
		Server,
		Code2,
		Wrench,
		Loader2,
		Check,
		X
	} from 'lucide-svelte';

	interface Props {
		id: string;
		title: string;
		description: string;
		category: string;
		packageCount: number;
		packages: string[];
		servicesCount: number;
		onInstall: () => void;
		onViewDetail: () => void;
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
		onViewDetail,
		installState = 'idle'
	}: Props = $props();

	const categoryIcons: Record<string, any> = {
		security: Shield,
		development: Code2,
		server: Server,
		tools: Wrench,
		default: Package
	};

	const categoryColors: Record<string, string> = {
		security: 'from-red-500/20 to-red-600/5 border-red-500/30',
		development: 'from-blue-500/20 to-blue-600/5 border-blue-500/30',
		server: 'from-purple-500/20 to-purple-600/5 border-purple-500/30',
		tools: 'from-amber-500/20 to-amber-600/5 border-amber-500/30',
		default: 'from-green-500/20 to-green-600/5 border-green-500/30'
	};

	const categoryBadgeColors: Record<string, string> = {
		security: 'bg-red-500/20 text-red-400 border-red-500/30',
		development: 'bg-blue-500/20 text-blue-400 border-blue-500/30',
		server: 'bg-purple-500/20 text-purple-400 border-purple-500/30',
		tools: 'bg-amber-500/20 text-amber-400 border-amber-500/30',
		default: 'bg-green-500/20 text-green-400 border-green-500/30'
	};

	let IconComponent = $derived(categoryIcons[category] || categoryIcons['default']);
	let gradientClass = $derived(categoryColors[category] || categoryColors['default']);
	let badgeClass = $derived(categoryBadgeColors[category] || categoryBadgeColors['default']);
</script>

<div
	class="group relative bg-card border border-border rounded-2xl overflow-hidden transition-all duration-300 hover:border-primary/40 hover:shadow-lg hover:shadow-primary/5 hover:-translate-y-0.5"
>
	<!-- Gradient Header -->
	<div class="h-2 bg-gradient-to-r {gradientClass}"></div>

	<div class="p-6">
		<!-- Top Row: Icon + Category Badge -->
		<div class="flex items-start justify-between mb-4">
			<div
				class="w-12 h-12 rounded-xl bg-gradient-to-br {gradientClass} flex items-center justify-center border"
			>
				<IconComponent class="w-6 h-6 text-foreground/80" />
			</div>
			<span class="text-xs font-medium px-2.5 py-1 rounded-full border {badgeClass}">
				{category}
			</span>
		</div>

		<!-- Title & Description -->
		<h3 class="font-bold text-lg mb-1.5 text-foreground group-hover:text-primary transition-colors">
			{title}
		</h3>
		<p class="text-muted-foreground text-sm leading-relaxed mb-4 line-clamp-2">
			{description}
		</p>

		<!-- Stats Row -->
		<div class="flex items-center gap-4 mb-5 text-xs text-muted-foreground">
			<div class="flex items-center gap-1.5">
				<Package class="w-3.5 h-3.5" />
				<span>{packageCount} packages</span>
			</div>
			{#if servicesCount > 0}
				<div class="flex items-center gap-1.5">
					<Server class="w-3.5 h-3.5" />
					<span>{servicesCount} services</span>
				</div>
			{/if}
		</div>

		<!-- Package Preview Pills -->
		<div class="flex flex-wrap gap-1.5 mb-5">
			{#each packages.slice(0, 4) as pkg}
				<span
					class="text-[11px] font-mono px-2 py-0.5 rounded-md bg-muted text-muted-foreground border border-border"
				>
					{pkg}
				</span>
			{/each}
			{#if packages.length > 4}
				<span
					class="text-[11px] font-mono px-2 py-0.5 rounded-md bg-muted text-muted-foreground border border-border"
				>
					+{packages.length - 4} more
				</span>
			{/if}
		</div>

		<!-- Action Buttons -->
		<div class="grid grid-cols-2 gap-3">
			<button
				onclick={onInstall}
				disabled={installState === 'installing'}
				class="flex items-center justify-center gap-2 py-2.5 px-4 rounded-xl text-sm font-semibold transition-all duration-200
					{installState === 'success'
					? 'bg-green-500/20 text-green-400 border border-green-500/30 cursor-default'
					: installState === 'error'
						? 'bg-red-500/20 text-red-400 border border-red-500/30'
						: installState === 'installing'
							? 'bg-primary/20 text-primary border border-primary/30 cursor-wait'
							: 'bg-primary text-primary-foreground hover:bg-primary/90 active:scale-[0.98]'}"
			>
				{#if installState === 'installing'}
					<Loader2 class="w-4 h-4 animate-spin" />
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
			</button>
			<button
				onclick={onViewDetail}
				class="flex items-center justify-center gap-2 py-2.5 px-4 rounded-xl text-sm font-medium border border-input hover:bg-accent hover:text-accent-foreground transition-all duration-200 active:scale-[0.98]"
			>
				Details
				<ChevronRight class="w-4 h-4" />
			</button>
		</div>
	</div>
</div>
