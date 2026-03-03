<script lang="ts">
	import type { ProfileInfo } from '$lib/commands';
	import { getCategoryIcon } from '$lib/utils/category';
	import { Package, Download, Terminal } from '@lucide/svelte';

	interface Props {
		profile: ProfileInfo;
		allInstalled: boolean;
		activeTab: 'preview' | 'package';
		onTabChange: (tab: 'preview' | 'package') => void;
	}

	let { profile, allInstalled, activeTab, onTabChange }: Props = $props();

	let IconComponent = $derived(getCategoryIcon(profile.category));
</script>

<div class="pt-container">
	<!-- ── Shared Header ──────────────────────────── -->
	<div class="pt-header">
		<div class="pt-header-left">
			<h1 class="pt-title">{profile.name}</h1>
			<p class="pt-desc">{profile.description}</p>
		</div>
		<div class="pt-tabs">
			<div class="bg-muted/50 p-1 rounded-lg border border-border flex">
				<button
					onclick={() => onTabChange('preview')}
					class={`px-4 py-1.5 rounded-md text-sm font-medium transition-all ${activeTab === 'preview' ? 'bg-background shadow-sm text-[#54CD4C]' : 'text-muted-foreground hover:text-foreground'}`}
				>
					Preview
				</button>
				<button
					onclick={() => onTabChange('package')}
					class={`px-4 py-1.5 rounded-md text-sm font-medium transition-all ${activeTab === 'package' ? 'bg-background shadow-sm text-[#54CD4C]' : 'text-muted-foreground hover:text-foreground'}`}
				>
					Package
				</button>
			</div>
		</div>
	</div>

	<hr class="pt-divider" />

	<!-- ── Preview Content ───────────────────────── -->
	{#if activeTab === 'preview'}
		<div class="pt-content">
			<div class="rounded-2xl bg-card border border-border p-6 space-y-6">
				<h2 class="text-lg font-semibold">Installation Preview</h2>

				<div class="space-y-6">
					<!-- Official Packages -->
					{#if profile.packages_install.length > 0}
						<div class="space-y-3">
							<h3 class="text-sm font-medium text-muted-foreground uppercase tracking-wider">
								Official Packages
							</h3>
							<div class="flex flex-wrap gap-2">
								{#each profile.packages_install as pkg}
									<div
										class="flex items-center gap-2 px-3 py-2 rounded-lg bg-muted/50 border border-border hover:bg-muted transition-colors"
									>
										<Package class="w-4 h-4 text-muted-foreground" />
										<span class="font-mono text-sm">{pkg}</span>
									</div>
								{/each}
							</div>
						</div>
					{/if}

					<!-- AUR Packages -->
					{#if profile.packages_aur.length > 0}
						<div class="space-y-3">
							<h3 class="text-sm font-medium text-muted-foreground uppercase tracking-wider">
								AUR Packages
							</h3>
							<div class="flex flex-wrap gap-2">
								{#each profile.packages_aur as pkg}
									<div
										class="flex items-center gap-2 px-3 py-2 rounded-lg bg-card border border-border border-l-2 border-l-amber-500/50 hover:bg-muted/30 transition-colors"
									>
										<Download class="w-4 h-4 text-amber-500" />
										<span class="font-mono text-sm">{pkg}</span>
									</div>
								{/each}
							</div>
						</div>
					{/if}

					<!-- Services -->
					{#if profile.services_enable.length > 0}
						<div class="space-y-3">
							<h3 class="text-sm font-medium text-muted-foreground uppercase tracking-wider">
								Services configuration
							</h3>
							<div class="flex flex-wrap gap-2">
								{#each profile.services_enable as svc}
									<div
										class="flex items-center gap-2 px-3 py-2 rounded-lg bg-card border border-border border-l-2 border-l-blue-500/50 hover:bg-muted/30 transition-colors"
									>
										<Terminal class="w-4 h-4 text-blue-500" />
										<span class="font-mono text-sm">systemctl enable {svc}</span>
									</div>
								{/each}
							</div>
						</div>
					{/if}
				</div>
			</div>
		</div>
	{/if}
</div>

<style>
	/* ── Container ───────────────────────────────── */
	.pt-container {
		display: flex;
		flex-direction: column;
	}

	/* ── Header ──────────────────────────────────── */
	.pt-header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 1.5rem;
		padding: 1.75rem 2rem 1.25rem;
	}

	.pt-header-left {
		display: flex;
		flex-direction: column;
		gap: 0.3rem;
	}

	.pt-title {
		font-size: 1.45rem;
		font-weight: 800;
		color: hsl(var(--foreground));
		margin: 0;
		letter-spacing: -0.02em;
		line-height: 1.15;
	}

	.pt-desc {
		font-size: 0.8rem;
		color: hsl(var(--muted-foreground));
		margin: 0;
		line-height: 1.55;
		max-width: 22rem;
	}

	/* ── Tab Switcher ────────────────────────────── */
	.pt-tabs {
		display: flex;
		align-items: center;
		gap: 0;
		padding: 0.2rem;
		border-radius: 0.6rem;
		background: hsl(var(--muted) / 0.4);
		border: 1px solid hsl(var(--border));
		flex-shrink: 0;
	}

	.pt-tab {
		padding: 0.45rem 1.1rem;
		border-radius: 0.45rem;
		font-size: 0.78rem;
		font-weight: 600;
		border: none;
		cursor: pointer;
		background: transparent;
		color: hsl(var(--muted-foreground));
		transition: all 0.18s ease;
		white-space: nowrap;
	}

	.pt-tab:hover:not(.pt-tab--active) {
		color: hsl(var(--foreground));
	}

	.pt-tab--active {
		background: #22c55e;
		color: #052e16;
		box-shadow: 0 1px 6px rgba(34, 197, 94, 0.35);
	}

	/* ── Divider ─────────────────────────────────── */
	.pt-divider {
		border: none;
		border-top: 1px solid hsl(var(--border) / 0.5);
		margin: 0 2rem;
	}

	/* ── Preview Content ─────────────────────────── */
	.pt-content {
		display: flex;
		flex-direction: column;
		gap: 1.4rem;
		padding: 1.5rem 2rem 2rem;
	}

	@media (max-width: 640px) {
		.pt-header {
			flex-direction: column;
			padding: 1.25rem 1.25rem 1rem;
		}

		.pt-content {
			padding: 1.25rem;
		}

		.pt-divider {
			margin: 0 1.25rem;
		}
	}
</style>
