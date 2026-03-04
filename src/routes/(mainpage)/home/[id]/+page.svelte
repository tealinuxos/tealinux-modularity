<script lang="ts">
	import { page } from '$app/state';
	import {
		commands,
		type ProfileInfo,
		type BackendResult,
		type PackageSizeInfo
	} from '$lib/commands';
	import { onMount } from 'svelte';
	import { Loader2, AlertTriangle } from 'lucide-svelte';
	import { goto } from '$app/navigation';

	import PreviewTab from '$lib/components/home/PreviewTab.svelte';
	import PackageListTab from '$lib/components/home/PackageListTab.svelte';
	import InstallActionBar from '$lib/components/home/InstallActionBar.svelte';
	import HeroDetail from '$lib/components/home/HeroDetail.svelte';
	// ─── Props & State ────────────────────────────────────────────────────────────
	let profileId = $derived(page.params.id);
	let profile = $state<ProfileInfo | null>(null);
	let loading = $state(true);
	let error = $state('');

	// Tab state
	let activeTab: 'preview' | 'package' = $state('package');

	// Package selection & status
	let selectedPackages: Set<string> = $state(new Set());
	let installedPackages: Set<string> = $state(new Set());
	let checkingPackages = $state(false);

	// Install state
	let installState: 'idle' | 'installing' | 'success' | 'error' = $state('idle');
	let installMessage = $state('');

	// Uninstall state
	let selectedUninstallPackages: Set<string> = $state(new Set());
	let uninstallState: 'idle' | 'uninstalling' | 'success' | 'error' = $state('idle');
	let uninstallMessage = $state('');
	let showForceConfirm = $state(false);
	let hasDependencyError = $state(false);

	// ─── Lifecycle ────────────────────────────────────────────────────────────────
	onMount(async () => {
		await loadProfile();
	});

	async function loadProfile() {
		loading = true;
		error = '';
		try {
			if (!profileId) throw new Error('No profile ID provided');
			profile = await commands.getProfile(profileId);

			if (profile) {
				profile.packages_install.forEach((p) => selectedPackages.add(p));
				checkPackagesStatus(profile.packages_install);
			} else {
				error = 'Profile not found';
			}
		} catch (e) {
			console.error('Error loading profile:', e);
			error = String(e);
		} finally {
			loading = false;
		}
	}

	async function checkPackagesStatus(packages: string[]) {
		checkingPackages = true;
		try {
			const results = await Promise.all(
				packages.map(async (pkg) => {
					const isInstalled = await commands.checkPackageInstalled(pkg);
					return { pkg, isInstalled };
				})
			);

			for (const { pkg, isInstalled } of results) {
				if (isInstalled) {
					installedPackages.add(pkg);
					selectedPackages.delete(pkg);
				}
			}
			// Trigger reactivity
			installedPackages = new Set(installedPackages);
			selectedPackages = new Set(selectedPackages);
		} catch (e) {
			console.error('Error checking package status:', e);
		} finally {
			checkingPackages = false;
		}
	}

	function togglePackage(pkg: string) {
		if (installedPackages.has(pkg)) return;

		if (selectedPackages.has(pkg)) {
			selectedPackages.delete(pkg);
		} else {
			selectedPackages.add(pkg);
		}
		selectedPackages = new Set(selectedPackages);
	}

	function toggleUninstallPackage(pkg: string) {
		if (!installedPackages.has(pkg)) return;

		if (selectedUninstallPackages.has(pkg)) {
			selectedUninstallPackages.delete(pkg);
		} else {
			selectedUninstallPackages.add(pkg);
		}
		selectedUninstallPackages = new Set(selectedUninstallPackages);
	}

	async function handleInstall() {
		if (!profile || selectedPackages.size === 0) return;

		installState = 'installing';
		installMessage = '';

		try {
			const packagesToInstall = Array.from(selectedPackages);
			console.log(`[install] Installing packages: ${packagesToInstall.join(', ')}`);

			const result: BackendResult = await commands.installProfile(
				profile.id,
				packagesToInstall,
				profile.services_enable
			);

			if (result.success) {
				installState = 'success';
				installMessage = result.stdout || 'Installation completed successfully.';
				await checkPackagesStatus(packagesToInstall);
			} else {
				installState = 'error';
				installMessage = result.stderr || 'Installation failed.';
			}
		} catch (e) {
			console.error('Install error:', e);
			installState = 'error';
			installMessage = String(e);
		}
	}

	async function handleUninstall(force: boolean = false) {
		if (!profile || selectedUninstallPackages.size === 0) return;

		uninstallState = 'uninstalling';
		uninstallMessage = '';
		showForceConfirm = false;
		hasDependencyError = false;

		try {
			const packagesToRemove = Array.from(selectedUninstallPackages);
			console.log(`[uninstall] Removing packages (force=${force}): ${packagesToRemove.join(', ')}`);

			const result: BackendResult = await commands.removePackages(packagesToRemove, force);

			// Re-check actual status of each package (handles partial success)
			for (const pkg of packagesToRemove) {
				const stillInstalled = await commands.checkPackageInstalled(pkg);
				if (!stillInstalled) {
					// Package was successfully removed
					installedPackages.delete(pkg);
					selectedUninstallPackages.delete(pkg);
					selectedPackages.add(pkg);
				}
			}
			installedPackages = new Set(installedPackages);
			selectedPackages = new Set(selectedPackages);
			selectedUninstallPackages = new Set(selectedUninstallPackages);

			if (result.success) {
				uninstallState = 'success';
				uninstallMessage = result.stdout || 'Packages removed successfully.';

				setTimeout(() => {
					uninstallState = 'idle';
				}, 2000);
			} else {
				// Check if it's a dependency error (offer force-remove)
				const isDependencyIssue =
					result.stderr.includes('depend on it') ||
					result.stderr.includes('required by') ||
					result.stderr.includes('Cannot remove');

				hasDependencyError = isDependencyIssue && !force;

				const hasSuccesses = result.stdout.includes('✓');
				uninstallState = 'error';
				uninstallMessage = hasSuccesses
					? 'Some packages removed. ' + result.stderr
					: result.stderr || 'Uninstall failed.';
			}
		} catch (e) {
			console.error('Uninstall error:', e);
			uninstallState = 'error';
			uninstallMessage = String(e);
		}
	}

	async function handleForceUninstall() {
		await handleUninstall(true);
	}

	async function handleUninstallAll() {
		if (!profile) return;

		// Select all installed packages for uninstall
		for (const pkg of installedPackages) {
			selectedUninstallPackages.add(pkg);
		}
		selectedUninstallPackages = new Set(selectedUninstallPackages);

		// Then trigger uninstall
		await handleUninstall();
	}

	// Computed stats
	let selectedCount = $derived(selectedPackages.size);
	let uninstallCount = $derived(selectedUninstallPackages.size);
	let allInstalled = $derived(
		installedPackages.size > 0 &&
			installedPackages.size === (profile?.packages_install?.length ?? 0)
	);

	// Real package size state
	let sizeInfo = $state<PackageSizeInfo | null>(null);
	let sizeLoading = $state(false);

	// Fetch real sizes whenever the selection changes
	$effect(() => {
		const pkgsToFetch = Array.from(selectedPackages);
		if (pkgsToFetch.length === 0) {
			sizeInfo = null;
			return;
		}
		sizeLoading = true;
		commands
			.getPackageSizes(pkgsToFetch)
			.then((info) => {
				sizeInfo = info;
			})
			.catch((e) => {
				console.error('[get_package_sizes] error:', e);
				sizeInfo = null;
			})
			.finally(() => {
				sizeLoading = false;
			});
	});

	// Derived display strings
	let totalDownloadSize = $derived(
		sizeLoading
			? 'Fetching...'
			: sizeInfo
				? sizeInfo.total_download_human
				: selectedCount > 0
					? 'Unknown'
					: '—'
	);
	let totalInstallSize = $derived(sizeLoading ? '' : sizeInfo ? sizeInfo.total_install_human : '');
	// Estimate time: assume 25 Mbps connection
	let estTime = $derived(
		!sizeInfo || sizeLoading
			? '—'
			: (() => {
					const seconds = sizeInfo.total_download_bytes / ((25 * 1024 * 1024) / 8);
					if (seconds < 60) return '<1 min';
					const mins = Math.ceil(seconds / 60);
					return `~${mins} min${mins > 1 ? 's' : ''}`;
				})()
	);
</script>

<div class="flex flex-col h-full bg-background relative overflow-hidden">
	{#if profile && !loading && !error}
		<HeroDetail
			{profile}
			{allInstalled}
			onInstall={handleInstall}
			onUninstall={handleUninstallAll}
		/>
	{/if}
	<div class="flex-1 overflow-y-auto">
		{#if loading}
			<div class="flex flex-col items-center justify-center h-full gap-4">
				<Loader2 class="w-8 h-8 text-primary animate-spin" />
				<p class="text-muted-foreground">Loading profile...</p>
			</div>
		{:else if error || !profile}
			<div class="flex flex-col items-center justify-center h-full gap-4">
				<AlertTriangle class="w-10 h-10 text-destructive" />
				<p class="text-destructive font-medium">{error || 'Profile not found'}</p>
				<button
					onclick={() => goto('/home')}
					class="px-4 py-2 bg-primary text-primary-foreground rounded-lg"
				>
					Go Back
				</button>
			</div>
		{:else}
			<div class="p-6 max-w-5xl mx-auto">
				<div class="rounded-3xl bg-card border border-border shadow-2xl overflow-hidden relative">
					<PreviewTab
						{profile}
						{allInstalled}
						{activeTab}
						onTabChange={(tab) => (activeTab = tab)}
					/>

					{#if activeTab === 'package'}
						<div class="p-8 pt-0">
							<PackageListTab
								packages={profile.packages_install}
								{selectedPackages}
								{installedPackages}
								{selectedUninstallPackages}
								onToggle={togglePackage}
								onToggleUninstall={toggleUninstallPackage}
							/>
						</div>
					{/if}
				</div>
			</div>
		{/if}
	</div>

	{#if profile && !loading && !error}
		<InstallActionBar
			{selectedCount}
			{uninstallCount}
			totalSize={totalDownloadSize}
			{totalInstallSize}
			{sizeLoading}
			{estTime}
			{installState}
			{installMessage}
			{uninstallState}
			{uninstallMessage}
			{hasDependencyError}
			onInstall={handleInstall}
			onUninstall={() => handleUninstall()}
			onForceUninstall={handleForceUninstall}
			onCancel={() => goto('/home')}
		/>
	{/if}
</div>
