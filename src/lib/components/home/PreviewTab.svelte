<script lang="ts">
	import type { ProfileInfo } from '$lib/commands';
	import { getCategoryIcon } from '$lib/utils/category';
	import { getDeviconClass } from '$lib/utils/devicon';
	import { Package, Download, Terminal, Copy, Check } from 'lucide-svelte';

	interface Props {
		profile: ProfileInfo;
		allInstalled: boolean;
		activeTab: 'preview' | 'package';
		onTabChange: (tab: 'preview' | 'package') => void;
	}

	let { profile, allInstalled, activeTab, onTabChange }: Props = $props();

	let IconComponent = $derived(getCategoryIcon(profile.category));

	// ── Service verification guides ──────────────────────────────────────
	type ServiceCommand = {
		label: string;
		cmd: string;
		type: 'setup' | 'maintenance';
	};

	type ServiceGuide = {
		description: string;
		commands: ServiceCommand[];
	};

	const serviceGuides: Record<string, ServiceGuide> = {
		docker: {
			description: 'Container runtime. Verify the daemon is running and list active containers.',
			commands: [
				{ label: 'Check Docker version', cmd: 'docker version', type: 'maintenance' },
				{ label: 'List running containers', cmd: 'docker ps', type: 'maintenance' },
				{ label: 'List all containers', cmd: 'docker ps -a', type: 'maintenance' },
				{
					label: 'Add user to docker group',
					cmd: 'sudo usermod -aG docker $USER && newgrp docker',
					type: 'setup'
				}
			]
		},
		postgresql: {
			description:
				'Relational database. Requires initdb on first boot before the service can start.',
			commands: [
				{ label: 'Check PostgreSQL version', cmd: 'psql --version', type: 'maintenance' },
				{ label: 'Check service status', cmd: 'systemctl status postgresql', type: 'maintenance' },
				{
					label: 'Initialize database (first time only)',
					cmd: 'sudo -u postgres initdb -D /var/lib/postgres/data',
					type: 'setup'
				},
				{ label: 'Connect to PostgreSQL', cmd: 'sudo -u postgres psql', type: 'maintenance' }
			]
		},
		redis: {
			description: 'In-memory key-value store for caching and sessions.',
			commands: [
				{ label: 'Check Redis version', cmd: 'redis-cli --version', type: 'maintenance' },
				{ label: 'Check service status', cmd: 'systemctl status redis', type: 'maintenance' },
				{ label: 'Ping Redis daemon', cmd: 'redis-cli ping', type: 'maintenance' },
				{ label: 'Monitor live traffic', cmd: 'redis-cli monitor', type: 'maintenance' }
			]
		},
		nginx: {
			description: 'High-performance web server and reverse proxy.',
			commands: [
				{ label: 'Check nginx version', cmd: 'nginx -v', type: 'maintenance' },
				{ label: 'Validate configuration', cmd: 'sudo nginx -t', type: 'setup' },
				{ label: 'Check service status', cmd: 'systemctl status nginx', type: 'maintenance' },
				{ label: 'Test HTTP response', cmd: 'curl -I http://localhost', type: 'maintenance' }
			]
		}
	};

	// ── Copy-to-clipboard state ──────────────────────────────────────────
	let copiedCmd = $state<string | null>(null);
	let copyTimeout: ReturnType<typeof setTimeout> | null = null;

	async function copyToClipboard(cmd: string) {
		try {
			await navigator.clipboard.writeText(cmd);
		} catch {
			// Fallback for environments without clipboard API
			const ta = document.createElement('textarea');
			ta.value = cmd;
			ta.style.position = 'fixed';
			ta.style.opacity = '0';
			document.body.appendChild(ta);
			ta.select();
			document.execCommand('copy');
			document.body.removeChild(ta);
		}
		copiedCmd = cmd;
		if (copyTimeout) clearTimeout(copyTimeout);
		copyTimeout = setTimeout(() => {
			copiedCmd = null;
		}, 1500);
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

					<!-- ── Service Configuration Guide ─────────────── -->
					{#if profile.services_enable.length > 0}
						<div class="space-y-3">
							<h3 class="text-sm font-medium text-muted-foreground uppercase tracking-wider">
								Services Configuration
							</h3>

							<div class="scg-grid">
								{#each profile.services_enable as svc}
									{@const guide = serviceGuides[svc]}
									<div class="scg-card">
										<!-- Card Header -->
										<div class="scg-card-header">
											<div class="scg-card-icon">
												<Terminal class="w-5 h-5" />
											</div>
											<div class="scg-card-meta">
												<h4 class="scg-card-name">{svc}</h4>
												{#if guide}
													<p class="scg-card-desc">{guide.description}</p>
												{/if}
											</div>
										</div>

										<!-- Enable command (always shown) -->
										<div class="scg-enable-block">
											<span class="scg-enable-comment"># Enable and start the service</span>
											<div class="scg-code-block">
												<code class="scg-code">systemctl enable --now {svc}</code>
												<button
													class="scg-copy-btn"
													class:scg-copy-btn--copied={copiedCmd ===
														`systemctl enable --now ${svc}`}
													onclick={() =>
														copyToClipboard(`systemctl enable --now ${svc}`)}
												>
													{#if copiedCmd === `systemctl enable --now ${svc}`}
														<Check class="w-3.5 h-3.5" />
														<span>Copied!</span>
													{:else}
														<Copy class="w-3.5 h-3.5" />
														<span>Copy</span>
													{/if}
												</button>
											</div>
										</div>

										<!-- Stepper Commands -->
										{#if guide}
											<div class="scg-stepper">
												{#each guide.commands as step, i}
													<div
														class="scg-step"
														class:scg-step--last={i === guide.commands.length - 1}
													>
														<!-- Dot + Line -->
														<div class="scg-step-track">
															<div
																class="scg-step-dot"
																class:scg-step-dot--setup={step.type === 'setup'}
															></div>
															{#if i < guide.commands.length - 1}
																<div class="scg-step-line"></div>
															{/if}
														</div>

														<!-- Content -->
														<div class="scg-step-content">
															<div class="scg-step-header">
																<span class="scg-step-label">{step.label}</span>
																<span
																	class="scg-badge"
																	class:scg-badge--setup={step.type === 'setup'}
																	class:scg-badge--maint={step.type ===
																		'maintenance'}
																>
																	{step.type === 'setup'
																		? 'Setup'
																		: 'Maintenance'}
																</span>
															</div>
															<div class="scg-code-block">
																<code class="scg-code">{step.cmd}</code>
																<button
																	class="scg-copy-btn"
																	class:scg-copy-btn--copied={copiedCmd ===
																		step.cmd}
																	onclick={() => copyToClipboard(step.cmd)}
																>
																	{#if copiedCmd === step.cmd}
																		<Check class="w-3.5 h-3.5" />
																		<span>Copied!</span>
																	{:else}
																		<Copy class="w-3.5 h-3.5" />
																		<span>Copy</span>
																	{/if}
																</button>
															</div>
														</div>
													</div>
												{/each}
											</div>
										{:else}
											<!-- Fallback for services without a guide -->
											<div class="scg-no-guide">
												<p class="scg-no-guide-text">
													No specific guide available. The service will be enabled
													automatically during installation.
												</p>
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

	/* ═══════════════════════════════════════════════
	   Service Configuration Guide (SCG) — Premium UI
	   ═══════════════════════════════════════════════ */

	.scg-grid {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	/* ── Card ──────────────────────────────────────── */
	.scg-card {
		border: 1px solid hsl(var(--border));
		border-radius: 0.875rem;
		background: hsl(var(--card));
		overflow: hidden;
		transition: box-shadow 0.2s ease, border-color 0.2s ease;
	}

	.scg-card:hover {
		border-color: hsl(var(--border) / 0.8);
		box-shadow:
			0 4px 24px -4px hsl(var(--foreground) / 0.04),
			0 1px 4px hsl(var(--foreground) / 0.02);
	}

	/* ── Card Header ───────────────────────────────── */
	.scg-card-header {
		display: flex;
		align-items: flex-start;
		gap: 0.875rem;
		padding: 1.25rem 1.25rem 0.75rem;
	}

	.scg-card-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 2.25rem;
		height: 2.25rem;
		border-radius: 0.5rem;
		background: hsl(217 91% 60% / 0.1);
		color: hsl(217 91% 60%);
		flex-shrink: 0;
	}

	.scg-card-meta {
		display: flex;
		flex-direction: column;
		gap: 0.15rem;
		min-width: 0;
	}

	.scg-card-name {
		margin: 0;
		font-family: 'Inter', ui-sans-serif, system-ui, sans-serif;
		font-size: 0.95rem;
		font-weight: 700;
		color: hsl(var(--foreground));
		text-transform: capitalize;
		letter-spacing: -0.01em;
	}

	.scg-card-desc {
		margin: 0;
		font-family: 'Inter', ui-sans-serif, system-ui, sans-serif;
		font-size: 0.78rem;
		color: hsl(var(--muted-foreground));
		line-height: 1.5;
	}

	/* ── Enable Block ──────────────────────────────── */
	.scg-enable-block {
		margin: 0 1.25rem;
		padding-bottom: 0.875rem;
		border-bottom: 1px solid hsl(var(--border) / 0.5);
	}

	.scg-enable-comment {
		display: block;
		font-family: 'JetBrains Mono', ui-monospace, 'Cascadia Code', 'Fira Code', monospace;
		font-size: 0.68rem;
		color: hsl(var(--muted-foreground) / 0.6);
		margin-bottom: 0.35rem;
		letter-spacing: 0.01em;
	}

	/* ── Code Block ────────────────────────────────── */
	.scg-code-block {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.75rem;
		background: #0a0a0a;
		border: 1px solid hsl(0 0% 100% / 0.08);
		border-radius: 0.5rem;
		padding: 0.65rem 0.75rem;
		position: relative;
		transition: border-color 0.15s ease;
	}

	.scg-code-block:hover {
		border-color: hsl(0 0% 100% / 0.14);
	}

	.scg-code {
		font-family: 'JetBrains Mono', ui-monospace, 'Cascadia Code', 'Fira Code', monospace;
		font-size: 0.78rem;
		color: #e2e8f0;
		line-height: 1.5;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
		min-width: 0;
	}

	/* ── Copy Button ───────────────────────────────── */
	.scg-copy-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.3rem;
		padding: 0.3rem 0.55rem;
		border-radius: 0.35rem;
		border: 1px solid hsl(0 0% 100% / 0.1);
		background: hsl(0 0% 100% / 0.05);
		color: #94a3b8;
		font-family: 'Inter', ui-sans-serif, system-ui, sans-serif;
		font-size: 0.68rem;
		font-weight: 500;
		cursor: pointer;
		flex-shrink: 0;
		transition: all 0.15s ease;
		white-space: nowrap;
	}

	.scg-copy-btn:hover {
		background: hsl(0 0% 100% / 0.1);
		color: #e2e8f0;
		border-color: hsl(0 0% 100% / 0.18);
	}

	.scg-copy-btn--copied {
		color: #4ade80 !important;
		border-color: hsl(142 71% 45% / 0.3) !important;
		background: hsl(142 71% 45% / 0.08) !important;
	}

	/* ── Vertical Stepper ──────────────────────────── */
	.scg-stepper {
		display: flex;
		flex-direction: column;
		padding: 1rem 1.25rem 1.25rem;
	}

	.scg-step {
		display: flex;
		gap: 1rem;
		position: relative;
	}

	/* Track (dot + line) */
	.scg-step-track {
		display: flex;
		flex-direction: column;
		align-items: center;
		flex-shrink: 0;
		width: 1.125rem;
		padding-top: 0.35rem;
	}

	.scg-step-dot {
		width: 0.5rem;
		height: 0.5rem;
		border-radius: 9999px;
		background: hsl(217 91% 60% / 0.6);
		border: 2px solid hsl(217 91% 60% / 0.2);
		flex-shrink: 0;
		position: relative;
		z-index: 1;
		transition: all 0.2s ease;
	}

	.scg-step-dot--setup {
		background: hsl(142 71% 45% / 0.7);
		border-color: hsl(142 71% 45% / 0.25);
	}

	.scg-step-line {
		width: 1.5px;
		flex: 1;
		background: hsl(var(--border) / 0.5);
		margin: 0.25rem 0;
	}

	/* Step content */
	.scg-step-content {
		display: flex;
		flex-direction: column;
		gap: 0.4rem;
		padding-bottom: 1.25rem;
		flex: 1;
		min-width: 0;
	}

	.scg-step--last .scg-step-content {
		padding-bottom: 0;
	}

	.scg-step-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.scg-step-label {
		font-family: 'Inter', ui-sans-serif, system-ui, sans-serif;
		font-size: 0.8rem;
		font-weight: 600;
		color: hsl(var(--foreground));
		letter-spacing: -0.005em;
	}

	/* ── Badges ────────────────────────────────────── */
	.scg-badge {
		display: inline-flex;
		align-items: center;
		padding: 0.1rem 0.45rem;
		border-radius: 9999px;
		font-family: 'Inter', ui-sans-serif, system-ui, sans-serif;
		font-size: 0.62rem;
		font-weight: 600;
		letter-spacing: 0.03em;
		text-transform: uppercase;
		line-height: 1.6;
	}

	.scg-badge--setup {
		background: hsl(142 71% 45% / 0.12);
		color: hsl(142 71% 45%);
		border: 1px solid hsl(142 71% 45% / 0.2);
	}

	.scg-badge--maint {
		background: hsl(217 91% 60% / 0.1);
		color: hsl(217 91% 60%);
		border: 1px solid hsl(217 91% 60% / 0.15);
	}

	/* ── No Guide Fallback ─────────────────────────── */
	.scg-no-guide {
		padding: 1rem 1.25rem 1.25rem;
	}

	.scg-no-guide-text {
		font-family: 'Inter', ui-sans-serif, system-ui, sans-serif;
		font-size: 0.78rem;
		color: hsl(var(--muted-foreground));
		margin: 0;
		line-height: 1.55;
		font-style: italic;
	}

	/* ── Responsive ────────────────────────────────── */
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

		.scg-card-header {
			padding: 1rem 1rem 0.65rem;
		}

		.scg-enable-block {
			margin: 0 1rem;
		}

		.scg-stepper {
			padding: 0.875rem 1rem 1rem;
		}

		.scg-step {
			gap: 0.75rem;
		}

		.scg-code {
			font-size: 0.72rem;
		}
	}
</style>
