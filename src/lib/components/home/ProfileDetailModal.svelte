<script lang="ts">
	import {
		X,
		Package,
		Server,
		Download,
		Loader2,
		Check,
		AlertTriangle,
		Shield,
		Code2,
		Wrench,
		CheckCircle2
	} from 'lucide-svelte';

	interface ProfileData {
		id: string;
		name: string;
		description: string;
		version: string;
		author: string;
		category: string;
		packages_install: string[];
		packages_aur: string[];
		services_enable: string[];
		package_count: number;
	}

	interface Props {
		profile: ProfileData | null;
		open: boolean;
		onClose: () => void;
		onInstall: (profile: ProfileData) => void;
		installState: 'idle' | 'installing' | 'success' | 'error';
		installMessage: string;
	}

	let {
		profile,
		open,
		onClose,
		onInstall,
		installState = 'idle',
		installMessage = ''
	}: Props = $props();

	const categoryIcons: Record<string, any> = {
		security: Shield,
		development: Code2,
		server: Server,
		tools: Wrench,
		default: Package
	};

	let IconComponent = $derived(
		profile ? categoryIcons[profile.category] || categoryIcons['default'] : Package
	);
</script>

{#if open && profile}
	<!-- Backdrop -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4"
		onclick={onClose}
	>
		<!-- Modal -->
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="bg-card border border-border rounded-2xl w-full max-w-lg max-h-[85vh] overflow-hidden shadow-2xl animate-in fade-in zoom-in-95 duration-200"
			onclick={(e) => e.stopPropagation()}
		>
			<!-- Header -->
			<div class="relative px-6 pt-6 pb-4 border-b border-border">
				<div class="flex items-start gap-4">
					<div
						class="w-14 h-14 rounded-xl bg-primary/10 flex items-center justify-center border border-primary/20 flex-shrink-0"
					>
						<IconComponent class="w-7 h-7 text-primary" />
					</div>
					<div class="flex-1 min-w-0">
						<h2 class="text-xl font-bold text-foreground">{profile.name}</h2>
						<p class="text-muted-foreground text-sm mt-0.5">{profile.description}</p>
						<div class="flex items-center gap-3 mt-2 text-xs text-muted-foreground">
							<span class="px-2 py-0.5 rounded-full bg-muted border border-border font-medium">
								v{profile.version}
							</span>
							<span>by {profile.author || 'TeaLinuxOS'}</span>
						</div>
					</div>
					<button
						onclick={onClose}
						class="p-1.5 rounded-lg hover:bg-muted transition-colors flex-shrink-0"
					>
						<X class="w-5 h-5 text-muted-foreground" />
					</button>
				</div>
			</div>

			<!-- Content -->
			<div class="px-6 py-4 overflow-y-auto max-h-[50vh] space-y-5">
				<!-- Official Packages -->
				{#if profile.packages_install.length > 0}
					<div>
						<div class="flex items-center gap-2 mb-3">
							<Package class="w-4 h-4 text-primary" />
							<h3 class="text-sm font-semibold text-foreground">
								Official Packages ({profile.packages_install.length})
							</h3>
						</div>
						<div class="flex flex-wrap gap-1.5">
							{#each profile.packages_install as pkg}
								<span
									class="text-xs font-mono px-2.5 py-1 rounded-lg bg-muted text-foreground border border-border hover:border-primary/30 transition-colors"
								>
									{pkg}
								</span>
							{/each}
						</div>
					</div>
				{/if}

				<!-- AUR Packages -->
				{#if profile.packages_aur.length > 0}
					<div>
						<div class="flex items-center gap-2 mb-3">
							<Download class="w-4 h-4 text-amber-400" />
							<h3 class="text-sm font-semibold text-foreground">
								AUR Packages ({profile.packages_aur.length})
							</h3>
						</div>
						<div class="flex flex-wrap gap-1.5">
							{#each profile.packages_aur as pkg}
								<span
									class="text-xs font-mono px-2.5 py-1 rounded-lg bg-amber-500/10 text-amber-300 border border-amber-500/20"
								>
									{pkg}
								</span>
							{/each}
						</div>
					</div>
				{/if}

				<!-- Services -->
				{#if profile.services_enable.length > 0}
					<div>
						<div class="flex items-center gap-2 mb-3">
							<Server class="w-4 h-4 text-blue-400" />
							<h3 class="text-sm font-semibold text-foreground">
								Services to Enable ({profile.services_enable.length})
							</h3>
						</div>
						<div class="flex flex-wrap gap-1.5">
							{#each profile.services_enable as svc}
								<span
									class="text-xs font-mono px-2.5 py-1 rounded-lg bg-blue-500/10 text-blue-300 border border-blue-500/20"
								>
									{svc}
								</span>
							{/each}
						</div>
					</div>
				{/if}

				<!-- Install Status Message -->
				{#if installState !== 'idle'}
					<div
						class="rounded-xl p-4 border {installState === 'success'
							? 'bg-green-500/10 border-green-500/20'
							: installState === 'error'
								? 'bg-red-500/10 border-red-500/20'
								: 'bg-primary/10 border-primary/20'}"
					>
						<div class="flex items-start gap-3">
							{#if installState === 'installing'}
								<Loader2 class="w-5 h-5 text-primary animate-spin flex-shrink-0 mt-0.5" />
								<div>
									<p class="text-sm font-medium text-foreground">Installing profile...</p>
									<p class="text-xs text-muted-foreground mt-1">
										This may take a while. A password prompt may appear.
									</p>
								</div>
							{:else if installState === 'success'}
								<CheckCircle2 class="w-5 h-5 text-green-400 flex-shrink-0 mt-0.5" />
								<div>
									<p class="text-sm font-medium text-green-400">Installation successful!</p>
									<p class="text-xs text-muted-foreground mt-1">{installMessage}</p>
								</div>
							{:else if installState === 'error'}
								<AlertTriangle class="w-5 h-5 text-red-400 flex-shrink-0 mt-0.5" />
								<div>
									<p class="text-sm font-medium text-red-400">Installation failed</p>
									<p class="text-xs text-muted-foreground mt-1">{installMessage}</p>
								</div>
							{/if}
						</div>
					</div>
				{/if}
			</div>

			<!-- Footer Actions -->
			<div class="px-6 py-4 border-t border-border flex gap-3">
				{#if installState === 'success'}
					<button
						onclick={onClose}
						class="flex-1 flex items-center justify-center gap-2 py-3 px-4 rounded-xl text-sm font-semibold bg-green-600 text-white hover:bg-green-700 active:scale-[0.98] transition-all duration-200 shadow-lg shadow-green-900/20"
					>
						<Check class="w-5 h-5" />
						Done
					</button>
				{:else}
					<button
						onclick={() => profile && onInstall(profile)}
						disabled={installState === 'installing'}
						class="flex-1 flex items-center justify-center gap-2 py-3 px-4 rounded-xl text-sm font-semibold transition-all duration-200
							{installState === 'error'
							? 'bg-red-500/10 text-red-500 border border-red-500/20 hover:bg-red-500/20'
							: installState === 'installing'
								? 'bg-primary/20 text-primary border border-primary/30 cursor-wait'
								: 'bg-primary text-primary-foreground hover:bg-primary/90 active:scale-[0.98]'}"
					>
						{#if installState === 'installing'}
							<Loader2 class="w-4 h-4 animate-spin" />
							Installing...
						{:else if installState === 'error'}
							<AlertTriangle class="w-4 h-4" />
							Retry Install
						{:else}
							<Download class="w-5 h-5" />
							Install Profile
						{/if}
					</button>
					<button
						onclick={onClose}
						disabled={installState === 'installing'}
						class="px-6 py-3 rounded-xl text-sm font-medium border border-input hover:bg-accent transition-colors {installState ===
						'installing'
							? 'opacity-50 cursor-not-allowed'
							: ''}"
					>
						Close
					</button>
				{/if}
			</div>
		</div>
	</div>
{/if}
