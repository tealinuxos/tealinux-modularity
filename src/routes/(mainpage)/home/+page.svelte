<script lang="ts">
	import Hero from '$lib/components/home/Hero.svelte';
	import ProfileCard from '$lib/components/home/ProfileCard.svelte';

	import { commands, type ProfileInfo } from '$lib/commands';
	import { onMount } from 'svelte';
	import { LoaderCircle, RefreshCw, CircleAlert } from '@lucide/svelte';

	// ── Global install state — persists across navigation ──────────────────
	import { installStore } from '$lib/stores/install.svelte';
	import { installPackages, syncActiveTasks } from '$lib/services/installManager';

	// ─── State ────────────────────────────────────────────────────────────────────
	let profiles: ProfileInfo[] = $state([]);
	let loading = $state(true);
	let loadError = $state('');

	// ─── Load profiles from backend on mount ──────────────────────────────────────
	onMount(async () => {
		await syncActiveTasks();
		await loadProfiles();
	});

	async function loadProfiles() {
		loading = true;
		loadError = '';
		try {
			profiles = await commands.listProfiles();
		} catch (e) {
			console.error('Failed to load profiles:', e);
			loadError = String(e);
		} finally {
			loading = false;
		}
	}

	// ── Install handler — uses global async streaming system ──────────────────
	async function handleCardInstall(profile: ProfileInfo) {
		if (installStore.isBusy) return;

		const officialPackages = profile.packages_install;
		const aurPackages = profile.packages_aur;

		if (aurPackages.length > 0) {
			console.warn(`[install] Skipping AUR packages for now: ${aurPackages.join(', ')}`);
		}

		// Use the global streaming install system
		await installPackages(profile.id, profile.name, officialPackages, profile.services_enable);
	}

	// ── Derive install state per profile from global store ─────────────────────
	function getProfileInstallState(profileId: string): 'idle' | 'installing' | 'success' | 'error' {
		if (installStore.activeInstall?.profileId === profileId) {
			const phase = installStore.activeInstall.phase;
			if (phase === 'installing') return 'installing';
			if (phase === 'success') return 'success';
			if (phase === 'error') return 'error';
		}
		const stored = installStore.getProfilePhase(profileId);
		if (stored === 'installing') return 'installing';
		if (stored === 'success') return 'success';
		if (stored === 'error') return 'error';
		return 'idle';
	}
</script>

<div class="flex h-full flex-col gap-7 overflow-y-auto p-7">
	<!-- Hero Section -->
	<Hero profileCount={profiles.length} />

	<!-- Loading State -->
	{#if loading}
		<div class="flex flex-col items-center justify-center py-20 gap-4">
			<LoaderCircle class="w-8 h-8 text-primary animate-spin" />
			<p class="text-muted-foreground text-sm">Loading profiles...</p>
		</div>
	{:else if loadError}
		<!-- Error State -->
		<div
			class="flex flex-col items-center justify-center py-16 gap-4 bg-destructive/5 rounded-2xl border border-destructive/20"
		>
			<CircleAlert class="w-10 h-10 text-destructive" />
			<div class="text-center space-y-1">
				<p class="text-foreground font-semibold">Failed to load profiles</p>
				<p class="text-muted-foreground text-sm max-w-md">{loadError}</p>
			</div>
			<button
				onclick={loadProfiles}
				class="flex items-center gap-2 px-4 py-2 rounded-lg bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 transition-colors"
			>
				<RefreshCw class="w-4 h-4" />
				Try Again
			</button>
		</div>
	{:else if profiles.length === 0}
		<!-- Empty State -->
		<div class="flex flex-col items-center justify-center py-20 gap-3">
			<div class="w-16 h-16 rounded-2xl bg-muted flex items-center justify-center">
				<CircleAlert class="w-8 h-8 text-muted-foreground" />
			</div>
			<p class="text-foreground font-semibold">No profiles found</p>
			<p class="text-muted-foreground text-sm">Add .toml profiles to the profiles/ directory.</p>
		</div>
	{:else}
		<!-- Profiles Grid -->
		<div class="grid grid-cols-1 gap-6 md:grid-cols-2">
			{#each profiles as profile (profile.id)}
				<ProfileCard
					id={profile.id}
					title={profile.name}
					description={profile.description}
					category={profile.category}
					packageCount={profile.package_count}
					packages={[...profile.packages_install, ...profile.packages_aur]}
					servicesCount={profile.services_enable.length}
					installState={getProfileInstallState(profile.id)}
					onInstall={() => handleCardInstall(profile)}
				/>
			{/each}
		</div>
	{/if}
</div>

<!-- Profile Detail Modal -->
