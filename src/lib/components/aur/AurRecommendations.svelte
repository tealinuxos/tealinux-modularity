<script lang="ts">
	import { onMount } from 'svelte';
	import { TrendingUp, Loader2, RefreshCw } from '@lucide/svelte';
	import AurPackageCard from './AurPackageCard.svelte';
	import Pagination from '$lib/components/ui/Pagination.svelte';
	import { commands } from '$lib/commands';
	import type { AurPackageInfo } from '$lib/commands';

	interface Props {
		getAurState: (name: string) => 'idle' | 'installing' | 'uninstalling' | 'success' | 'error';
		oninstall: (name: string) => void;
		onremove: (name: string) => void;
	}

	let { getAurState, oninstall, onremove }: Props = $props();

	let popularPackages: AurPackageInfo[] = $state([]);
	let loading = $state(true);
	let error = $state('');

	let currentPage = $state(1);
	const itemsPerPage = 12;
	let totalPages = $derived(Math.ceil(popularPackages.length / itemsPerPage));
	let paginatedPackages = $derived(
		popularPackages.slice((currentPage - 1) * itemsPerPage, currentPage * itemsPerPage)
	);

	onMount(async () => {
		await loadPopular();
	});

	async function loadPopular() {
		loading = true;
		error = '';
		currentPage = 1;
		try {
			popularPackages = await commands.getTopAurPackages();
			if (popularPackages.length === 0) {
				error = 'No popular packages found';
			}
		} catch (e) {
			console.error('[aur] Failed to load popular AUR pkgs', e);
			error = String(e);
		} finally {
			loading = false;
		}
	}
</script>

<div class="flex flex-col gap-6 animate-in fade-in slide-in-from-bottom-2 duration-500">
	<div class="flex items-center justify-between">
		<div class="flex items-center gap-3">
			<div
				class="w-8 h-8 rounded-lg bg-[#26A768]/10 border border-[#26A768]/20 flex items-center justify-center"
			>
				<TrendingUp class="w-4 h-4 text-[#26A768]" />
			</div>
			<div class="flex flex-col">
				<h2 class="text-sm font-bold text-foreground m-0">Popular AUR Packages</h2>
				<p class="text-[0.7rem] text-muted-foreground m-0">Frequently used community packages</p>
			</div>
		</div>
		{#if error}
			<button
				onclick={loadPopular}
				class="flex cursor-pointer border border-[#26A768]/30 bg-[#26A768]/10 items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-bold text-[#26A768] hover:bg-[#26A768]/20 transition-colors"
			>
				<RefreshCw class="w-[0.9rem] h-[0.9rem]" /> Retry
			</button>
		{/if}
	</div>

	{#if loading}
		<div class="flex flex-col items-center justify-center py-10 gap-3">
			<Loader2 class="w-6 h-6 text-[#26A768] animate-spin" />
			<p class="text-xs text-muted-foreground font-medium uppercase tracking-widest">
				Loading Top Packages...
			</p>
		</div>
	{:else if popularPackages.length > 0}
		<div class="grid grid-cols-1 xl:grid-cols-2 gap-4">
			{#each paginatedPackages as pkg (pkg.name)}
				<AurPackageCard
					{pkg}
					installState={getAurState(pkg.name)}
					{oninstall}
					{onremove}
				/>
			{/each}
		</div>

		<Pagination bind:currentPage {totalPages} />
	{/if}
</div>
