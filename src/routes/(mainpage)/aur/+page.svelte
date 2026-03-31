<script lang="ts">
	import AurSearchBar from '$lib/components/aur/AurSearchBar.svelte';
	import AurPackageCard from '$lib/components/aur/AurPackageCard.svelte';
	import AurPackageDetail from '$lib/components/aur/AurPackageDetail.svelte';
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
		CheckCircle2
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
	let selectedPackage: any = $state(null);
	let detailOpen = $state(false);
	let detailInstallState: 'idle' | 'installing' | 'success' | 'error' = $state('idle');
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
		detailInstallState = 'installing';
		try {
			const result = await commands.installAurPackage(name);
			if (result.success) {
				actionStates[name] = 'success';
				detailInstallState = 'success';
				searchResults = searchResults.map((pkg: any) =>
					pkg.name === name ? { ...pkg, installed: true } : pkg
				);
				await loadInstalledPackages();
			} else {
				actionStates[name] = 'error';
				detailInstallState = 'error';
			}
		} catch (e) {
			actionStates[name] = 'error';
			detailInstallState = 'error';
		}
		setTimeout(() => {
			actionStates[name] = 'idle';
			detailInstallState = 'idle';
		}, 4000);
	}

	async function handleRemove(name: string) {
		actionStates[name] = 'installing';
		detailInstallState = 'installing';
		try {
			const result = await commands.removeAurPackage(name);
			if (result.success) {
				actionStates[name] = 'success';
				detailInstallState = 'success';
				searchResults = searchResults.map((pkg: any) =>
					pkg.name === name ? { ...pkg, installed: false } : pkg
				);
				await loadInstalledPackages();
			} else {
				actionStates[name] = 'error';
				detailInstallState = 'error';
			}
		} catch (e) {
			actionStates[name] = 'error';
			detailInstallState = 'error';
		}
		setTimeout(() => {
			actionStates[name] = 'idle';
			detailInstallState = 'idle';
		}, 4000);
	}

	function openDetail(pkg: any) {
		selectedPackage = pkg;
		detailInstallState = actionStates[pkg.name] || 'idle';
		detailOpen = true;
	}

	function closeDetail() {
		detailOpen = false;
		selectedPackage = null;
	}
</script>

<div class="flex flex-col h-full bg-background relative">
	<div class="flex-1 overflow-y-auto">
		<div class="p-6 max-w-5xl mx-auto">
			<div
				class="flex flex-col rounded-3xl bg-card border border-border shadow-2xl overflow-hidden relative"
			>
				<!-- Header -->
				<div
					class="flex items-start justify-between gap-6 px-8 pt-7 pb-5 max-sm:flex-col max-sm:px-5 max-sm:pt-5 max-sm:pb-4"
				>
					<div class="flex items-center gap-4">
						<div
							class="flex items-center justify-center w-[3.25rem] h-[3.25rem] rounded-[0.875rem] shrink-0 bg-primary/10"
						>
							<Package class="w-7 h-7 text-primary" />
						</div>
						<div class="flex flex-col gap-1">
							<h1
								class="text-[1.35rem] font-extrabold text-foreground m-0 tracking-[-0.015em] leading-[1.2]"
							>
								AUR Packages
							</h1>
							<p class="text-[0.8rem] leading-relaxed text-muted-foreground m-0">
								Search, install, and manage Arch User Repository packages
							</p>
						</div>
					</div>
					{#if installedPackages.length > 0}
						<div
							class="flex shrink-0 items-center justify-center px-3 py-1.5 rounded-lg bg-accent text-accent-foreground text-xs font-medium"
						>
							{installedPackages.length} AUR package{installedPackages.length !== 1 ? 's' : ''} installed
						</div>
					{/if}
				</div>

				<hr class="mx-8 border-t border-border/50 max-sm:mx-5" />

				<div class="flex flex-col gap-5 px-8 pt-6 pb-8 max-sm:p-5">
					<!-- Tabs -->
					<div class="flex items-center gap-1 p-1 rounded-lg bg-accent/50 w-fit">
						<button
							id="aur-tab-search"
							onclick={() => (activeTab = 'search')}
							class="flex items-center gap-2 px-4 py-2 rounded-md text-sm font-medium transition-all {activeTab ===
							'search'
								? 'bg-background text-foreground shadow-sm'
								: 'text-muted-foreground hover:text-foreground'}"
						>
							<Search class="w-4 h-4" /> Search
						</button>
						<button
							id="aur-tab-installed"
							onclick={() => {
								activeTab = 'installed';
								loadInstalledPackages();
							}}
							class="flex items-center gap-2 px-4 py-2 rounded-md text-sm font-medium transition-all {activeTab ===
							'installed'
								? 'bg-background text-foreground shadow-sm'
								: 'text-muted-foreground hover:text-foreground'}"
						>
							<Download class="w-4 h-4" /> Installed
							{#if installedPackages.length > 0}
								<span
									class="px-1.5 py-0.5 rounded-full bg-primary/10 text-primary text-[10px] font-semibold"
									>{installedPackages.length}</span
								>
							{/if}
						</button>
					</div>

					<!-- Search Tab -->
					{#if activeTab === 'search'}
						<AurSearchBar onsearch={handleSearch} loading={searchLoading} />

						{#if searchLoading}
							<div class="flex flex-col items-center justify-center py-16 gap-4">
								<Loader2 class="w-8 h-8 text-primary animate-spin" />
								<p class="text-muted-foreground text-sm">Searching AUR...</p>
							</div>
						{:else if searchError}
							<div
								class="flex flex-col items-center justify-center py-12 gap-4 bg-destructive/5 rounded-2xl border border-destructive/20"
							>
								<AlertCircle class="w-10 h-10 text-destructive" />
								<div class="text-center space-y-1">
									<p class="text-foreground font-semibold">Search failed</p>
									<p class="text-muted-foreground text-sm max-w-md">{searchError}</p>
								</div>
								<button
									onclick={() => handleSearch(searchQuery)}
									class="flex items-center gap-2 px-4 py-2 rounded-lg bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 transition-colors"
								>
									<RefreshCw class="w-4 h-4" /> Try Again
								</button>
							</div>
						{:else if hasSearched && searchResults.length === 0}
							<div class="flex flex-col items-center justify-center py-16 gap-3">
								<div class="w-14 h-14 rounded-2xl bg-muted flex items-center justify-center">
									<Search class="w-7 h-7 text-muted-foreground" />
								</div>
								<p class="text-foreground font-semibold">No packages found</p>
								<p class="text-muted-foreground text-sm">Try a different search term</p>
							</div>
						{:else if searchResults.length > 0}
							<p class="text-xs text-muted-foreground">
								{searchResults.length} result{searchResults.length !== 1 ? 's' : ''} for "{searchQuery}"
							</p>
							<div class="grid grid-cols-1 md:grid-cols-2 gap-3">
								{#each searchResults as pkg (pkg.name)}
									<AurPackageCard
										{pkg}
										installState={actionStates[pkg.name] || 'idle'}
										oninstall={handleInstall}
										onremove={handleRemove}
										onclick={() => openDetail(pkg)}
									/>
								{/each}
							</div>
						{:else}
							<div class="flex flex-col items-center justify-center py-20 gap-4">
								<div
									class="w-16 h-16 rounded-2xl bg-primary/5 border border-primary/10 flex items-center justify-center"
								>
									<Package class="w-8 h-8 text-primary/40" />
								</div>
								<div class="text-center space-y-1">
									<p class="text-foreground font-semibold">Search the AUR</p>
									<p class="text-muted-foreground text-sm max-w-sm">
										Type a package name above to search the Arch User Repository.
									</p>
								</div>
							</div>
						{/if}
					{/if}

					<!-- Installed Tab -->
					{#if activeTab === 'installed'}
						{#if installedLoading}
							<div class="flex flex-col items-center justify-center py-16 gap-4">
								<Loader2 class="w-8 h-8 text-primary animate-spin" />
								<p class="text-muted-foreground text-sm">Loading installed packages...</p>
							</div>
						{:else if installedPackages.length === 0}
							<div class="flex flex-col items-center justify-center py-16 gap-3">
								<div class="w-14 h-14 rounded-2xl bg-muted flex items-center justify-center">
									<Package class="w-7 h-7 text-muted-foreground" />
								</div>
								<p class="text-foreground font-semibold">No AUR packages installed</p>
								<p class="text-muted-foreground text-sm">
									Search and install packages from the AUR
								</p>
							</div>
						{:else}
							<div class="flex items-center justify-between mb-2">
								<p class="text-xs text-muted-foreground">
									{installedPackages.length} AUR package{installedPackages.length !== 1 ? 's' : ''} installed
								</p>
								<button
									onclick={loadInstalledPackages}
									class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
								>
									<RefreshCw class="w-3.5 h-3.5" /> Refresh
								</button>
							</div>
							<div class="flex flex-col gap-2">
								{#each installedPackages as pkg (pkg.name)}
									<div
										class="flex items-center justify-between p-3 rounded-xl border border-border bg-card hover:border-primary/20 transition-colors"
									>
										<div class="flex items-center gap-3 min-w-0">
											<div
												class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center shrink-0"
											>
												<CheckCircle2 class="w-4 h-4 text-primary" />
											</div>
											<div class="min-w-0">
												<p class="text-sm font-medium text-foreground truncate">{pkg.name}</p>
												<p class="text-xs text-muted-foreground font-mono">{pkg.version}</p>
											</div>
										</div>
										<div class="shrink-0">
											{#if actionStates[pkg.name] === 'installing'}
												<div
													class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-primary/10 text-primary text-xs font-medium"
												>
													<Loader2 class="w-3.5 h-3.5 animate-spin" /> Removing...
												</div>
											{:else}
												<button
													onclick={() => handleRemove(pkg.name)}
													class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-destructive/10 text-destructive hover:bg-destructive/20 text-xs font-medium transition-colors"
													><Trash2 class="w-3.5 h-3.5" /> Remove</button
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

<AurPackageDetail
	pkg={selectedPackage}
	open={detailOpen}
	installState={detailInstallState}
	onclose={closeDetail}
	oninstall={handleInstall}
	onremove={handleRemove}
/>
