<script lang="ts">
	import type { ProfileInfo } from '$lib/commands';
	import { getCategoryIcon } from '$lib/utils/category';
	import { getDeviconClass } from '$lib/utils/devicon';
	import { Package, Download, Terminal, ChevronDown } from 'lucide-svelte';

	interface Props {
		profile: ProfileInfo;
		allInstalled: boolean;
		activeTab: 'preview' | 'package';
		onTabChange: (tab: 'preview' | 'package') => void;
	}

	let { profile, allInstalled, activeTab, onTabChange }: Props = $props();

	let IconComponent = $derived(getCategoryIcon(profile.category));

	// ── Service verification guides ──────────────────────────────────────
	type ServiceGuide = {
		description: string;
		commands: { label: string; cmd: string }[];
	};

	const serviceGuides: Record<string, ServiceGuide> = {
		docker: {
			description: 'Container runtime. Verify the daemon is running and list active containers.',
			commands: [
				{ label: 'Check version', cmd: 'docker version' },
				{ label: 'Running containers', cmd: 'docker ps' },
				{ label: 'All containers', cmd: 'docker ps -a' },
				{ label: 'Add user to group', cmd: 'sudo usermod -aG docker $USER && newgrp docker' }
			]
		},
		postgresql: {
			description:
				'Relational database. Requires initdb on first boot before the service can start.',
			commands: [
				{ label: 'Check version', cmd: 'psql --version' },
				{ label: 'Service status', cmd: 'systemctl status postgresql' },
				{ label: 'Init DB (first time)', cmd: 'sudo -u postgres initdb -D /var/lib/postgres/data' },
				{ label: 'Connect', cmd: 'sudo -u postgres psql' }
			]
		},
		redis: {
			description: 'In-memory key-value store for caching and sessions.',
			commands: [
				{ label: 'Check version', cmd: 'redis-cli --version' },
				{ label: 'Service status', cmd: 'systemctl status redis' },
				{ label: 'Ping daemon', cmd: 'redis-cli ping' },
				{ label: 'Monitor (live)', cmd: 'redis-cli monitor' }
			]
		},
		nginx: {
			description: 'High-performance web server and reverse proxy.',
			commands: [
				{ label: 'Check version', cmd: 'nginx -v' },
				{ label: 'Test config', cmd: 'sudo nginx -t' },
				{ label: 'Service status', cmd: 'systemctl status nginx' },
				{ label: 'Test response', cmd: 'curl -I http://localhost' }
			]
		}
	};

	// Track which service cards are expanded
	let expandedServices = $state<Set<string>>(new Set());

	function toggleService(svc: string) {
		const next = new Set(expandedServices);
		if (next.has(svc)) {
			next.delete(svc);
		} else {
			next.add(svc);
		}
		expandedServices = next;
	}
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
									{@const devIcon = getDeviconClass(pkg)}
									<div
										class="flex items-center gap-2 px-3 py-2 rounded-lg bg-muted/50 border border-border hover:bg-muted transition-colors"
									>
										{#if devIcon}
											<i class="{devIcon} text-lg text-muted-foreground flex-shrink-0"></i>
										{:else}
											<Package class="w-4 h-4 text-muted-foreground shrink-0" />
										{/if}
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
									{@const devIcon = getDeviconClass(pkg)}
									<div
										class="flex items-center gap-2 px-3 py-2 rounded-lg bg-card border border-border border-l-2 border-l-amber-500/50 hover:bg-muted/30 transition-colors"
									>
										{#if devIcon}
											<i class="{devIcon} text-lg text-amber-500 flex-shrink-0"></i>
										{:else}
											<Download class="w-4 h-4 text-amber-500 shrink-0" />
										{/if}
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
							<div class="svc-list">
								{#each profile.services_enable as svc}
									<div class="svc-card">
										<!-- Clickable header chip -->
										<button
											class="svc-chip"
											class:svc-chip--open={expandedServices.has(svc)}
											onclick={() => toggleService(svc)}
											disabled={!serviceGuides[svc]}
										>
											<Terminal class="w-4 h-4 text-blue-500 shrink-0" />
											<span class="font-mono text-sm">systemctl enable {svc}</span>
											{#if serviceGuides[svc]}
												<ChevronDown
													class="w-3.5 h-3.5 text-muted-foreground ml-auto shrink-0 svc-chevron"
													style={expandedServices.has(svc) ? 'transform:rotate(180deg)' : ''}
												/>
											{/if}
										</button>

										<!-- Expandable guide panel -->
										{#if expandedServices.has(svc) && serviceGuides[svc]}
											{@const guide = serviceGuides[svc]}
											<div class="svc-guide">
												<p class="svc-guide-desc">{guide.description}</p>
												<div class="svc-cmds">
													{#each guide.commands as { label, cmd }}
														<div class="svc-cmd-row">
															<span class="svc-cmd-label">{label}</span>
															<code class="svc-cmd-code">{cmd}</code>
														</div>
													{/each}
												</div>
											</div>
										{/if}
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

	/* ── Service Cards ───────────────────────────── */
	.svc-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.svc-card {
		display: flex;
		flex-direction: column;
		border-radius: 0.6rem;
		overflow: hidden;
	}

	.svc-chip {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: hsl(var(--card));
		border: 1px solid hsl(var(--border));
		border-left: 2px solid hsl(217 91% 60% / 0.5);
		border-radius: 0.6rem;
		cursor: default;
		text-align: left;
		width: 100%;
		transition: background 0.15s ease;
	}

	.svc-chip:not(:disabled) {
		cursor: pointer;
	}

	.svc-chip:not(:disabled):hover {
		background: hsl(var(--muted) / 0.3);
	}

	.svc-chip--open {
		border-bottom-left-radius: 0;
		border-bottom-right-radius: 0;
		background: hsl(var(--muted) / 0.2);
		border-left-color: hsl(217 91% 60% / 0.8);
	}

	.svc-chevron {
		transition: transform 0.2s ease;
	}

	.svc-guide {
		padding: 0.85rem 1rem;
		background: hsl(var(--muted) / 0.12);
		border: 1px solid hsl(var(--border));
		border-top: none;
		border-left: 2px solid hsl(217 91% 60% / 0.4);
		border-bottom-left-radius: 0.6rem;
		border-bottom-right-radius: 0.6rem;
		display: flex;
		flex-direction: column;
		gap: 0.65rem;
	}

	.svc-guide-desc {
		font-size: 0.75rem;
		color: hsl(var(--muted-foreground));
		margin: 0;
		line-height: 1.5;
	}

	.svc-cmds {
		display: flex;
		flex-direction: column;
		gap: 0.35rem;
	}

	.svc-cmd-row {
		display: flex;
		align-items: baseline;
		gap: 0.75rem;
	}

	.svc-cmd-label {
		font-size: 0.7rem;
		color: hsl(var(--muted-foreground));
		min-width: 7rem;
		text-align: right;
		flex-shrink: 0;
	}

	.svc-cmd-code {
		font-family: ui-monospace, 'Cascadia Code', 'Fira Code', monospace;
		font-size: 0.72rem;
		padding: 0.2rem 0.5rem;
		border-radius: 0.3rem;
		background: hsl(var(--muted) / 0.5);
		border: 1px solid hsl(var(--border) / 0.6);
		color: hsl(var(--foreground));
		word-break: break-all;
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
