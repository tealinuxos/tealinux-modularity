<script lang="ts">
	import { page } from '$app/state';
	import { commands, type ProfileInfo, type BackendResult } from '$lib/commands';
	import { onMount } from 'svelte';
	import { Loader2, AlertTriangle } from 'lucide-svelte';
	import { goto } from '$app/navigation';

	import ProfileHeroCard from '$lib/components/home/ProfileHeroCard.svelte';
	import PreviewTab from '$lib/components/home/PreviewTab.svelte';
	import PackageListTab from '$lib/components/home/PackageListTab.svelte';
	import InstallActionBar from '$lib/components/home/InstallActionBar.svelte';

	// ─── Props & State ────────────────────────────────────────────────────────────
	let profileId = $derived(page.params.id);
	let profile: ProfileInfo | null = $state(null);
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

	// Computed stats
	let selectedCount = $derived(selectedPackages.size);
	let allInstalled = $derived(
		!!profile && installedPackages.size === profile.packages_install.length
	);
	let totalSize = $derived('~1.2 GB');
	let estTime = $derived('~' + Math.max(1, Math.ceil(selectedCount / 5)) + ' mins');
</script>

<div class="flex flex-col h-full bg-background relative overflow-hidden">
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
					onclick={() => goto('/')}
					class="px-4 py-2 bg-primary text-primary-foreground rounded-lg"
				>
					Go Back
				</button>
			</div>
		{:else}
			<div class="p-6 max-w-5xl mx-auto space-y-6">
				<ProfileHeroCard
					{profile}
					{allInstalled}
					{activeTab}
					onTabChange={(tab) => (activeTab = tab)}
				/>

				{#if activeTab === 'preview'}
					<PreviewTab {profile} />
				{/if}

				{#if activeTab === 'package'}
					<PackageListTab
						packages={profile.packages_install}
						{selectedPackages}
						{installedPackages}
						onToggle={togglePackage}
					/>
				{/if}
			</div>
		{/if}
	</div>

	{#if profile && !loading && !error}
		<InstallActionBar
			{selectedCount}
			{totalSize}
			{estTime}
			{installState}
			{installMessage}
			onInstall={handleInstall}
			onCancel={() => goto('/')}
		/>
	{/if}
</div>
