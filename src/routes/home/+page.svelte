<script lang="ts">
	import Hero from '$lib/components/home/Hero.svelte';
	import ProfileCard from '$lib/components/home/ProfileCard.svelte';

	import { commands, type ProfileInfo, type BackendResult } from '$lib/commands';
	import { onMount } from 'svelte';
	import { Loader2, RefreshCw, AlertCircle } from 'lucide-svelte';

	// ─── State ────────────────────────────────────────────────────────────────────
	let profiles: ProfileInfo[] = $state([]);
	let loading = $state(true);
	let loadError = $state('');

	// Install states per profile id
	let installStates: Record<string, 'idle' | 'installing' | 'success' | 'error'> = $state({});

	// ─── Load profiles from backend on mount ──────────────────────────────────────
	onMount(async () => {
		await loadProfiles();
	});

	async function loadProfiles() {
		loading = true;
		loadError = '';
		try {
			profiles = await commands.listProfiles();
			// Initialize install states
			for (const p of profiles) {
				installStates[p.id] = 'idle';
			}
		} catch (e) {
			console.error('Failed to load profiles:', e);
			loadError = String(e);
		} finally {
			loading = false;
		}
	}

	// ─── Install handler ──────────────────────────────────────────────────────────
	async function handleInstall(profile: ProfileInfo) {
		installStates[profile.id] = 'installing';
		modalInstallState = 'installing';
		modalInstallMessage = '';

		try {
			// Only install official packages via backend (pacman)
			// AUR packages cannot be installed via pkexec pacman and need a separate user-level flow (yay/paru)
			const officialPackages = profile.packages_install;
			const aurPackages = profile.packages_aur;

			if (aurPackages.length > 0) {
				console.warn(`[install] Skipping AUR packages for now: ${aurPackages.join(', ')}`);
			}

			console.log(`[install] Profile: ${profile.name}`);
			console.log(`[install] Official Packages: ${officialPackages.join(', ')}`);
			console.log(`[install] Services: ${profile.services_enable.join(', ')}`);

			const result: BackendResult = await commands.installProfile(
				profile.id,
				officialPackages,
				profile.services_enable
			);

			console.log('[install] Result:', result);

			if (result.success) {
				installStates[profile.id] = 'success';
				modalInstallState = 'success';
				modalInstallMessage = result.stdout || `Profile "${profile.name}" installed successfully.`;
			} else {
				installStates[profile.id] = 'error';
				modalInstallState = 'error';
				modalInstallMessage = result.stderr || 'Unknown error occurred.';
			}
		} catch (e) {
			console.error('[install] Error:', e);
			installStates[profile.id] = 'error';
			modalInstallState = 'error';
			modalInstallMessage = String(e);
		}

		// Auto-reset error state after 5s so user can retry
		if (installStates[profile.id] === 'error') {
			setTimeout(() => {
				installStates[profile.id] = 'idle';
			}, 5000);
		}
	}

	function handleCardInstall(profile: ProfileInfo) {
		handleInstall(profile);
	}
</script>



<div class="flex flex-col gap-6 p-6 h-full overflow-y-auto">
	<!-- Hero Section -->
	<Hero profileCount={profiles.length} />

	<!-- Loading State -->
	{#if loading}
		<div class="flex flex-col items-center justify-center py-20 gap-4">
			<Loader2 class="w-8 h-8 text-primary animate-spin" />
			<p class="text-muted-foreground text-sm">Loading profiles...</p>
		</div>
	{:else if loadError}
		<!-- Error State -->
		<div
			class="flex flex-col items-center justify-center py-16 gap-4 bg-destructive/5 rounded-2xl border border-destructive/20"
		>
			<AlertCircle class="w-10 h-10 text-destructive" />
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
				<AlertCircle class="w-8 h-8 text-muted-foreground" />
			</div>
			<p class="text-foreground font-semibold">No profiles found</p>
			<p class="text-muted-foreground text-sm">Add .toml profiles to the profiles/ directory.</p>
		</div>
	{:else}
		<!-- Profiles Grid -->
		<div class="grid grid-cols-1 md:grid-cols-2 gap-5">
			{#each profiles as profile (profile.id)}
				<ProfileCard
					id={profile.id}
					title={profile.name}
					description={profile.description}
					category={profile.category}
					packageCount={profile.package_count}
					packages={[...profile.packages_install, ...profile.packages_aur]}
					servicesCount={profile.services_enable.length}
					installState={installStates[profile.id] || 'idle'}
					onInstall={() => handleCardInstall(profile)}
				/>
			{/each}
		</div>
	{/if}
</div>

<!-- Profile Detail Modal -->
