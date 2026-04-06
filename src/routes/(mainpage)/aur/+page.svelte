<script lang="ts">
	import AurSearchBar from '$lib/components/aur/AurSearchBar.svelte';
	import AurPackageCard from '$lib/components/aur/AurPackageCard.svelte';
	import AurRecommendations from '$lib/components/aur/AurRecommendations.svelte';
	import { commands } from '$lib/commands';
	import { onMount } from 'svelte';
	import {
		Loader2,
		AlertCircle,
		RefreshCw,
		Package,
		Search,
		Download,
		Trash2,
		CheckCircle2,
		Info
	} from '@lucide/svelte';

	type TabId = 'search' | 'installed';

	let activeTab: TabId = $state('search');
	let searchResults: any[] = $state([]);
	let searchLoading = $state(false);
	let searchQuery = $state('');
	let searchError = $state('');
	let hasSearched = $state(false);
	let installedPackages: any[] = $state([]);
	let installedLoading = $state(false);
	let actionStates: Record<string, 'idle' | 'installing' | 'success' | 'error'> = $state({});

	onMount(async () => {
		await loadInstalledPackages();
	});

	async function handleSearch(query: string) {
		searchQuery = query;
		if (!query || query.length < 2) {
			searchResults = [];
			hasSearched = false;
			return;
		}
		searchLoading = true;
		searchError = '';
		hasSearched = true;
		try {
			searchResults = await commands.searchAurPackages(query);
		} catch (e) {
			searchError = String(e);
			searchResults = [];
		} finally {
			searchLoading = false;
		}
	}

	async function loadInstalledPackages() {
		installedLoading = true;
		try {
			installedPackages = await commands.listInstalledAur();
		} catch (e) {
			console.error('[aur] Failed to load installed:', e);
		} finally {
			installedLoading = false;
		}
	}

	async function handleInstall(name: string) {
		actionStates[name] = 'installing';
		try {
			const result = await commands.installAurPackage(name);
			if (result.success) {
				actionStates[name] = 'success';
				searchResults = searchResults.map((pkg: any) =>
					pkg.name === name ? { ...pkg, installed: true } : pkg
				);
				await loadInstalledPackages();
			} else {
				actionStates[name] = 'error';
			}
		} catch (e) {
			actionStates[name] = 'error';
		}
		setTimeout(() => {
			actionStates[name] = 'idle';
		}, 4000);
	}

	async function handleRemove(name: string) {
		actionStates[name] = 'installing';
		try {
			const result = await commands.removeAurPackage(name);
			if (result.success) {
				actionStates[name] = 'success';
				searchResults = searchResults.map((pkg: any) =>
					pkg.name === name ? { ...pkg, installed: false } : pkg
				);
				await loadInstalledPackages();
			} else {
				actionStates[name] = 'error';
			}
		} catch (e) {
			actionStates[name] = 'error';
		}
		setTimeout(() => {
			actionStates[name] = 'idle';
		}, 4000);
	}
</script>

<div class="flex flex-col h-full bg-background relative">
	<div class="flex-1 overflow-y-auto">
		<div class="p-6 max-w-5xl mx-auto">
			<!-- Inline global alert at top to replace flameshot/toast logic -->
			<div
				class="mb-5 flex items-start gap-3 rounded-xl border border-yellow-500/20 bg-yellow-500/10 px-4 py-3 shadow-[0_4px_14px_rgba(234,179,8,0.06)]"
			>
				<Info class="w-5 h-5 text-yellow-600 dark:text-yellow-500 shrink-0 mt-0.5" />
				<div class="text-sm text-yellow-900 dark:text-yellow-200/90 leading-relaxed">
					<p class="font-bold mb-0.5">Note on AUR Packages</p>
					AUR packages are community-produced. Some packages may compile from source and take a relatively
					long time to build and install. Proceed with awareness.
				</div>
			</div>

			<div
				class="flex flex-col rounded-[1.5rem] bg-card border border-border/50 shadow-2xl overflow-hidden relative"
			>
				<!-- Header -->
				<div
					class="flex items-start justify-between gap-6 px-8 pt-7 pb-5 max-sm:flex-col max-sm:px-5 max-sm:pt-5 max-sm:pb-4"
				>
					<div class="flex items-center gap-4">
						<div
							class="flex items-center justify-center w-[3.25rem] h-[3.25rem] rounded-[0.875rem] shrink-0 bg-gradient-to-br from-[#26A768] to-emerald-600 shadow-[0_4px_14px_rgba(84,205,76,0.35)]"
						>
							<Package class="w-7 h-7 text-[#052e16]" />
						</div>
						<div class="flex flex-col gap-1">
							<div class="flex items-center gap-3">
								<h1
									class="text-[1.35rem] font-extrabold text-foreground m-0 tracking-[-0.015em] leading-[1.2]"
								>
									AUR Packages
								</h1>
								<span
									class="py-[0.15rem] px-[0.5rem] rounded-full text-[0.58rem] font-extrabold uppercase tracking-[0.08em] leading-[1.6] transition-all bg-[#26A768] text-[#052e16]"
								>
									SYNCED
								</span>
							</div>

							<p class="text-[0.8rem] leading-relaxed text-muted-foreground m-0">
								Search, install, and manage community packages
							</p>
						</div>
					</div>

					<!-- Segmented Control Tabs -->
					<div
						class="flex shrink-0 items-center rounded-lg border border-border/60 bg-muted/40 p-1 self-start sm:self-center shadow-sm"
					>
						<button
							id="aur-tab-search"
							onclick={() => (activeTab = 'search')}
							class="flex transition-all items-center gap-2 px-4 py-1.5 rounded-md text-sm font-medium {activeTab ===
							'search'
								? 'bg-background text-[#26A768] shadow-sm font-bold border border-border/30'
								: 'text-muted-foreground hover:text-foreground cursor-pointer border border-transparent'}"
						>
							<Search class="w-4 h-4" /> Search
						</button>
						<button
							id="aur-tab-installed"
							onclick={() => {
								activeTab = 'installed';
								loadInstalledPackages();
							}}
							class="flex transition-all items-center gap-2 px-4 py-1.5 rounded-md text-sm font-medium {activeTab ===
							'installed'
								? 'bg-background text-[#26A768] shadow-sm font-bold border border-border/30'
								: 'text-muted-foreground hover:text-foreground cursor-pointer border border-transparent'}"
						>
							<Download class="w-4 h-4" /> Installed
							{#if installedPackages.length > 0}
								<span
									class="px-1.5 py-0.5 rounded-sm bg-[#26A768]/10 text-[#26A768] text-[10px] font-bold"
									>{installedPackages.length}</span
								>
							{/if}
						</button>
					</div>
				</div>

				<hr class="mx-8 border-t border-border/50 max-sm:mx-5" />

				<div class="flex flex-col gap-6 px-8 pt-6 pb-8 max-sm:p-5">
					<!-- Search Tab -->
					{#if activeTab === 'search'}
						<AurSearchBar
							onsearch={handleSearch}
							loading={searchLoading}
							bind:value={searchQuery}
						/>

						{#if searchLoading}
							<div class="flex flex-col items-center justify-center py-16 gap-4">
								<Loader2 class="w-8 h-8 text-[#26A768] animate-spin" />
								<p class="text-muted-foreground text-sm font-medium uppercase tracking-widest">
									Searching AUR...
								</p>
							</div>
						{:else if searchError}
							<div
								class="flex flex-col items-center justify-center py-12 gap-4 bg-destructive/5 rounded-2xl border border-destructive/20"
							>
								<AlertCircle class="w-10 h-10 text-destructive shadow-sm" />
								<div class="text-center space-y-1">
									<p class="text-foreground font-bold">Search request failed</p>
									<p class="text-muted-foreground text-sm max-w-md">{searchError}</p>
								</div>
								<button
									onclick={() => handleSearch(searchQuery)}
									class="flex cursor-pointer items-center gap-2 px-4 py-2 rounded-lg bg-[#26A768] border-none text-[#052e16] text-sm font-bold hover:bg-[#4bc043] transition-colors shadow-[0_3px_12px_rgba(84,205,76,0.3)] hover:shadow-[0_4px_16px_rgba(84,205,76,0.4)]"
								>
									<RefreshCw class="w-4 h-4" /> Try Again
								</button>
							</div>
						{:else if hasSearched && searchResults.length === 0}
							<div class="flex flex-col items-center justify-center py-16 gap-3">
								<div
									class="w-14 h-14 rounded-2xl bg-muted/50 border border-border/50 flex items-center justify-center"
								>
									<Search class="w-7 h-7 text-muted-foreground/60" />
								</div>
								<p class="text-foreground font-bold">No packages found</p>
								<p class="text-muted-foreground text-sm">
									Try a different search term or check spelling
								</p>
							</div>
						{:else if searchResults.length > 0}
							<p
								class="text-[0.75rem] font-bold uppercase tracking-widest text-muted-foreground/80"
							>
								{searchResults.length} result{searchResults.length !== 1 ? 's' : ''} for "{searchQuery}"
							</p>
							<div class="grid grid-cols-1 xl:grid-cols-2 gap-4">
								{#each searchResults as pkg (pkg.name)}
									<AurPackageCard
										{pkg}
										installState={actionStates[pkg.name] || 'idle'}
										oninstall={handleInstall}
										onremove={handleRemove}
									/>
								{/each}
							</div>
						{:else}
							<AurRecommendations
								{actionStates}
								oninstall={handleInstall}
								onremove={handleRemove}
							/>
						{/if}
					{/if}

					<!-- Installed Tab -->
					{#if activeTab === 'installed'}
						{#if installedLoading}
							<div class="flex flex-col items-center justify-center py-16 gap-4">
								<Loader2 class="w-8 h-8 text-[#26A768] animate-spin" />
								<p class="text-muted-foreground text-[0.75rem] font-bold uppercase tracking-widest">
									Loading installed...
								</p>
							</div>
						{:else if installedPackages.length === 0}
							<div class="flex flex-col items-center justify-center py-16 gap-3">
								<div
									class="w-14 h-14 rounded-2xl bg-muted/50 border border-border/50 flex items-center justify-center"
								>
									<Package class="w-7 h-7 text-muted-foreground/60" />
								</div>
								<p class="text-foreground font-bold">No AUR packages installed</p>
								<p class="text-muted-foreground text-sm">
									You can search and install community packages from the Search tab
								</p>
							</div>
						{:else}
							<div class="flex items-center justify-between mb-2">
								<p
									class="text-[0.75rem] font-bold uppercase tracking-widest text-muted-foreground/80"
								>
									{installedPackages.length} package{installedPackages.length !== 1 ? 's' : ''} installed
								</p>
								<button
									onclick={loadInstalledPackages}
									class="flex cursor-pointer border-none bg-transparent items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-bold text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
								>
									<RefreshCw class="w-[0.9rem] h-[0.9rem]" /> Refresh
								</button>
							</div>
							<div class="flex flex-col gap-3">
								{#each installedPackages as pkg (pkg.name)}
									<div
										class="flex items-center justify-between p-4 rounded-xl border border-border/50 bg-card/60 hover:bg-card/80 hover:border-[#26A768]/30 hover:shadow-sm transition-all"
									>
										<div class="flex items-center gap-4 min-w-0">
											<div
												class="w-9 h-9 rounded-xl bg-[#26A768]/10 flex items-center justify-center shrink-0 border border-[#26A768]/20 shadow-[0_2px_10px_rgba(84,205,76,0.1)]"
											>
												<CheckCircle2 class="w-5 h-5 text-[#26A768]" />
											</div>
											<div class="min-w-0 flex flex-col gap-0.5">
												<p class="text-[0.95rem] font-bold text-foreground truncate">{pkg.name}</p>
												<div class="flex items-center gap-2">
													<p
														class="text-[0.7rem] text-muted-foreground font-mono bg-accent px-1.5 py-0.5 rounded-sm m-0"
													>
														{pkg.version}
													</p>
													<span
														class="text-[0.6rem] font-extrabold uppercase tracking-widest text-[#26A768]/90"
														>Installed</span
													>
												</div>
											</div>
										</div>
										<div class="shrink-0 flex items-center">
											{#if actionStates[pkg.name] === 'installing'}
												<div
													class="flex items-center gap-1.5 px-3 py-2 rounded-lg bg-[#26A768]/10 text-[#26A768] text-xs font-semibold uppercase tracking-widest"
												>
													<Loader2 class="w-[0.9rem] h-[0.9rem] animate-spin" /> Removing
												</div>
											{:else}
												<button
													onclick={() => handleRemove(pkg.name)}
													class="flex items-center gap-1.5 px-3 py-2 rounded-[0.55rem] border-none bg-red-500/10 text-red-500 hover:bg-red-500 hover:text-white hover:shadow-[0_4px_16px_rgba(239,68,68,0.3)] hover:-translate-y-px text-[0.75rem] font-extrabold transition-all cursor-pointer"
													><Trash2 class="w-[0.9rem] h-[0.9rem]" /> Remove</button
												>
											{/if}
										</div>
									</div>
								{/each}
							</div>
						{/if}
					{/if}
				</div>
			</div>
		</div>
	</div>
</div>
