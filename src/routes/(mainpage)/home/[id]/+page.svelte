<script lang="ts">
	import { page } from '$app/state';
	import { commands, type ProfileInfo, type PackageSizeInfo } from '$lib/commands';
	import { onMount } from 'svelte';
	import { LoaderCircle, TriangleAlert } from '@lucide/svelte';
	import { goto } from '$app/navigation';

	import PreviewTab from '$lib/components/home/PreviewTab.svelte';
	import PackageListTab from '$lib/components/home/PackageListTab.svelte';
	import InstallActionBar from '$lib/components/home/InstallActionBar.svelte';

	// ── Global install state (persists across navigation) ─────────────────────
	import { installStore } from '$lib/stores/install.svelte';
	import {
		installPackages,
		uninstallPackages,
		syncActiveTasks
	} from '$lib/services/installManager';

	// ── Profile data ──────────────────────────────────────────────────────────
	let profileId = $derived(page.params.id ?? '');
	let profile = $state<ProfileInfo | null>(null);
	let loading = $state(true);
	let error = $state('');

	// Tab state — local UI, fine to reset on navigation
	let activeTab: 'preview' | 'package' = $state('package');

	// Package selection & installed status — local to this page
	let selectedPackages: Set<string> = $state(new Set());
	let installedPackages: Set<string> = $state(new Set());
	let checkingPackages = $state(false);

	// Uninstall selection — local to this page
	let selectedUninstallPackages: Set<string> = $state(new Set());
	let hasDependencyError = $state(false);

	// ── Read install state from global store ──────────────────────────────────
	// These are DERIVED from the global store — no local copies
	let installState = $derived(
		installStore.activeInstall?.profileId === profileId
			? (installStore.activeInstall?.phase as 'idle' | 'installing' | 'success' | 'error')
			: (installStore.getProfilePhase(profileId) as
					| 'idle'
					| 'installing'
					| 'success'
					| 'error')
	);

	let installMessage = $derived(
		installStore.activeInstall?.profileId === profileId
			? (installStore.activeInstall?.message ?? '')
			: ''
	);

	let uninstallState = $derived(
		installStore.activeUninstall?.profileId === profileId
			? (installStore.activeUninstall?.phase as
					| 'idle'
					| 'uninstalling'
					| 'success'
					| 'error')
			: 'idle'
	);

	let uninstallMessage = $derived(
		installStore.activeUninstall?.profileId === profileId
			? (installStore.activeUninstall?.message ?? '')
			: ''
	);

	// ── Lifecycle ─────────────────────────────────────────────────────────────
	onMount(async () => {
		await syncActiveTasks();
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

	// ── Install handler — delegates to global installManager ──────────────────
	async function handleInstall() {
		if (!profile || !profileId || selectedPackages.size === 0) return;
		if (installStore.isBusy) return;

		const pkgs = Array.from(selectedPackages);

		// This returns immediately — progress updates arrive via global events
		await installPackages(profileId, profile.name, pkgs, profile.services_enable);

		// After the install finishes (detected via the store), refresh package status
		// We watch for phase change using $effect below
	}

	// When install completes, refresh installed status
	$effect(() => {
		const phase = installStore.activeInstall?.phase;
		const pid = installStore.activeInstall?.profileId;
		if (phase === 'success' && pid === profileId && profile) {
			// Refresh installed packages status
			checkPackagesStatus(profile.packages_install);
		}
	});

	// ── Uninstall handler — delegates to global installManager ────────────────
	async function handleUninstall(force: boolean = false) {
		if (!profile || !profileId || selectedUninstallPackages.size === 0) return;
		if (installStore.isBusy) return;

		hasDependencyError = false;
		const packagesToRemove = Array.from(selectedUninstallPackages);

		const result = await uninstallPackages(
			profileId,
			profile.name,
			packagesToRemove,
			profile.services_enable,
			force
		);

		hasDependencyError = result.hasDependencyError;

		if (result.success) {
			// Re-check actual status of each package (handles partial success)
			for (const pkg of packagesToRemove) {
				const stillInstalled = await commands.checkPackageInstalled(pkg);
				if (!stillInstalled) {
					installedPackages.delete(pkg);
					selectedUninstallPackages.delete(pkg);
					selectedPackages.add(pkg);
				}
			}
			installedPackages = new Set(installedPackages);
			selectedPackages = new Set(selectedPackages);
			selectedUninstallPackages = new Set(selectedUninstallPackages);
		}
	}

	async function handleForceUninstall() {
		await handleUninstall(true);
	}

	async function handleUninstallAll() {
		if (!profile) return;
		for (const pkg of installedPackages) {
			selectedUninstallPackages.add(pkg);
		}
		selectedUninstallPackages = new Set(selectedUninstallPackages);
		await handleUninstall();
	}

	// ── Computed stats ────────────────────────────────────────────────────────
	let selectedCount = $derived(selectedPackages.size);
	let uninstallCount = $derived(selectedUninstallPackages.size);
	let allInstalled = $derived(
		installedPackages.size > 0 &&
			installedPackages.size === (profile?.packages_install?.length ?? 0)
	);

	// Real package size state
	let sizeInfo = $state<PackageSizeInfo | null>(null);
	let sizeLoading = $state(false);

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

<div class="flex flex-col h-full bg-background relative">
	<div class="flex-1 overflow-y-auto">
		{#if loading}
			<div class="flex flex-col items-center justify-center h-full gap-4">
				<LoaderCircle class="w-8 h-8 text-primary animate-spin" />
				<p class="text-muted-foreground">Loading profile...</p>
			</div>
		{:else if error || !profile}
			<div class="flex flex-col items-center justify-center h-full gap-4">
				<TriangleAlert class="w-10 h-10 text-destructive" />
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
						onInstall={handleInstall}
						onUninstall={() => handleUninstall()}
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
