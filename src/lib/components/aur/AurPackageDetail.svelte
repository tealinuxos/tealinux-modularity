<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog/';
	import {
		Download,
		Trash2,
		ExternalLink,
		Star,
		TrendingUp,
		User,
		Calendar,
		Scale,
		GitBranch,
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
		url: string;
		aur_url: string;
		first_submitted?: number | bigint | null;
		last_modified?: number | bigint | null;
		license: string[];
		depends: string[];
		make_depends: string[];
		opt_depends: string[];
	}

	interface Props {
		pkg: AurPackage | null;
		open: boolean;
		installState?: 'idle' | 'installing' | 'uninstalling' | 'success' | 'error';
		onclose: () => void;
		oninstall?: (name: string) => void;
		onremove?: (name: string) => void;
	}

	let { pkg, open = false, installState = 'idle', onclose, oninstall, onremove }: Props = $props();

	function formatDate(timestamp?: number | bigint | null): string {
		if (!timestamp) return 'Unknown';
		return new Date(Number(timestamp) * 1000).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function formatPopularity(pop: number): string {
		if (pop >= 100) return pop.toFixed(0);
		if (pop >= 10) return pop.toFixed(1);
		return pop.toFixed(2);
	}

	function handleOpenChange(v: boolean) {
		if (!v) onclose();
	}
</script>

<div>
	{#if pkg && open}
		<Dialog.Root {open} onOpenChange={handleOpenChange}>
			<Dialog.Content class="max-w-lg max-h-[85vh] overflow-y-auto">
				<Dialog.Header>
					<div class="flex items-baseline gap-3">
						<Dialog.Title class="text-2xl font-bold">{pkg.name}</Dialog.Title>
						<span class="text-sm font-mono text-muted-foreground/40">{pkg.version}</span>
					</div>
					<div class="flex items-center gap-3 mt-2">
						{#if pkg.installed}
							<span
								class="flex items-center gap-1.5 text-[10px] font-bold uppercase tracking-widest text-[#26A768] bg-[#26A768]/10 px-2 py-0.5 rounded"
							>
								<CheckCircle2 class="w-3.5 h-3.5" /> Installed
							</span>
						{/if}
						{#if pkg.out_of_date}
							<span
								class="flex items-center gap-1.5 text-[10px] font-bold uppercase tracking-widest text-red-500 bg-red-500/10 px-2 py-0.5 rounded"
							>
								<AlertTriangle class="w-3.5 h-3.5" /> Out of date
							</span>
						{/if}
					</div>
				</Dialog.Header>

				<p class="mt-4 text-sm text-muted-foreground leading-relaxed">
					{pkg.description || 'No description available'}
				</p>

				<div class="grid grid-cols-2 gap-3 mt-4">
					<div class="flex items-center gap-2 p-2.5 rounded-lg bg-accent/50">
						<Star class="w-4 h-4 text-primary" />
						<div>
							<p class="text-xs text-muted-foreground">Votes</p>
							<p class="text-sm font-semibold text-foreground">{pkg.num_votes}</p>
						</div>
					</div>
					<div class="flex items-center gap-2 p-2.5 rounded-lg bg-accent/50">
						<TrendingUp class="w-4 h-4 text-primary" />
						<div>
							<p class="text-xs text-muted-foreground">Popularity</p>
							<p class="text-sm font-semibold text-foreground">
								{formatPopularity(pkg.popularity)}
							</p>
						</div>
					</div>
					<div class="flex items-center gap-2 p-2.5 rounded-lg bg-accent/50">
						<User class="w-4 h-4 text-primary" />
						<div>
							<p class="text-xs text-muted-foreground">Maintainer</p>
							<p class="text-sm font-semibold text-foreground truncate">{pkg.maintainer}</p>
						</div>
					</div>
					<div class="flex items-center gap-2 p-2.5 rounded-lg bg-accent/50">
						<Scale class="w-4 h-4 text-primary" />
						<div>
							<p class="text-xs text-muted-foreground">License</p>
							<p class="text-sm font-semibold text-foreground truncate">
								{pkg.license.length > 0 ? pkg.license.join(', ') : 'Unknown'}
							</p>
						</div>
					</div>
				</div>

				<div class="flex items-center gap-4 mt-3 text-xs text-muted-foreground">
					<span class="flex items-center gap-1"
						><Calendar class="w-3.5 h-3.5" /> Submitted: {formatDate(pkg.first_submitted)}</span
					>
					<span class="flex items-center gap-1"
						><Calendar class="w-3.5 h-3.5" /> Modified: {formatDate(pkg.last_modified)}</span
					>
				</div>

				{#if pkg.depends.length > 0}
					<div class="mt-4">
						<div class="flex items-center gap-1.5 mb-2">
							<GitBranch class="w-3.5 h-3.5 text-muted-foreground" />
							<p class="text-xs font-medium text-muted-foreground uppercase tracking-wide">
								Dependencies ({pkg.depends.length})
							</p>
						</div>
						<div class="flex flex-wrap gap-1.5">
							{#each pkg.depends.slice(0, 20) as dep}
								<span
									class="px-2 py-0.5 rounded text-[11px] font-mono bg-accent text-accent-foreground"
									>{dep}</span
								>
							{/each}
							{#if pkg.depends.length > 20}
								<span class="px-2 py-0.5 rounded text-[11px] text-muted-foreground"
									>+{pkg.depends.length - 20} more</span
								>
							{/if}
						</div>
					</div>
				{/if}

				<div class="flex items-center gap-3 mt-4 pt-3 border-t border-border">
					{#if pkg.url}
						<a
							href={pkg.url}
							target="_blank"
							rel="noopener noreferrer"
							class="flex items-center gap-1.5 text-xs text-primary hover:underline"
							><ExternalLink class="w-3.5 h-3.5" /> Upstream</a
						>
					{/if}
					<a
						href={pkg.aur_url}
						target="_blank"
						rel="noopener noreferrer"
						class="flex items-center gap-1.5 text-xs text-primary hover:underline"
						><ExternalLink class="w-3.5 h-3.5" /> AUR Page</a
					>
				</div>

				<div class="flex items-center gap-3 mt-4 pt-3 border-t border-border">
					{#if installState === 'installing'}
						<div
							class="flex items-center gap-2 px-4 py-2 rounded-lg bg-primary/10 text-primary text-sm font-medium w-full justify-center"
						>
							<Loader2 class="w-4 h-4 animate-spin" /> Installing...
						</div>
					{:else if installState === 'uninstalling'}
						<div
							class="flex items-center gap-2 px-4 py-2 rounded-lg bg-destructive/10 text-destructive text-sm font-medium w-full justify-center"
						>
							<Loader2 class="w-4 h-4 animate-spin" /> Uninstalling...
						</div>
					{:else if installState === 'success'}
						<div
							class="flex items-center gap-2 px-4 py-2 rounded-lg bg-green-500/10 text-green-600 dark:text-green-400 text-sm font-medium w-full justify-center"
						>
							<CheckCircle2 class="w-4 h-4" /> Installed successfully!
						</div>
					{:else if installState === 'error'}
						<div
							class="flex items-center gap-2 px-4 py-2 rounded-lg bg-destructive/10 text-destructive text-sm font-medium w-full justify-center"
						>
							<XCircle class="w-4 h-4" /> Installation failed
						</div>
					{:else if pkg.installed}
						<button
							onclick={() => onremove?.(pkg?.name ?? '')}
							class="flex items-center gap-2 px-4 py-2 rounded-lg bg-destructive/10 text-destructive hover:bg-destructive/20 text-sm font-medium transition-colors w-full justify-center"
							><Trash2 class="w-4 h-4" /> Remove Package</button
						>
					{:else}
						<button
							onclick={() => oninstall?.(pkg?.name ?? '')}
							class="flex items-center gap-2 px-4 py-2 rounded-lg bg-[#26A768] text-[#052e16] hover:bg-[#4bc043] shadow-[0_3px_12px_rgba(84,205,76,0.2)] text-sm font-bold transition-all w-full justify-center"
							><Download class="w-4 h-4" /> Install Package</button
						>
					{/if}
				</div>
			</Dialog.Content>
		</Dialog.Root>
	{/if}
</div>
