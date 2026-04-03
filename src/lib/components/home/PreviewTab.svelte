<script lang="ts">
	import type { ProfileInfo } from '$lib/commands';
	import PackageBadge from './PackageBadge.svelte';
	import ServiceGuideCard from './ServiceGuideCard.svelte';
	import { getCategoryIcon } from '$lib/utils/category';
	import { Package, Download, Terminal } from '@lucide/svelte';

	interface Props {
		profile: ProfileInfo;
		allInstalled: boolean;
		activeTab: 'preview' | 'package';
		onTabChange: (tab: 'preview' | 'package') => void;
		onInstall?: () => void;
		onUninstall?: () => void;
	}

	let { profile, allInstalled, activeTab, onTabChange, onInstall, onUninstall }: Props = $props();

	let IconComponent = $derived(profile ? getCategoryIcon(profile.category) : null);

	// ── Selected package state ────────────────────────────────────────────
	let selectedPackage = $state<string | null>(null);

	function handlePackageClick(pkg: string) {
		selectedPackage = selectedPackage === pkg ? null : pkg;
	}

	// ── Package verification guides ──────────────────────────────────────
	type ServiceCommand = {
		label: string;
		cmd: string;
		type: 'setup' | 'maintenance';
	};

	type ServiceGuide = {
		description: string;
		commands: ServiceCommand[];
	};

	const packageGuides = packageGuidesRaw as Record<string, ServiceGuide>;

	let selectedGuide = $derived(selectedPackage ? (packageGuides[selectedPackage] ?? null) : null);
	let isService = $derived(
		selectedPackage ? profile.services_enable.includes(selectedPackage) : false
	);

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


		<!-- Header -->
		<div
			class="flex items-start justify-between gap-6 px-8 pt-7 pb-5 max-sm:flex-col max-sm:px-5 max-sm:pt-5 max-sm:pb-4"
		>
			<!-- Left: icon + identity -->
			<div class="flex items-center gap-4">
				<div
					class="w-[3.25rem] h-[3.25rem] rounded-[0.875rem] shrink-0 flex items-center justify-center transition-all duration-300 bg-gradient-to-br {allInstalled
						? 'from-red-500 to-red-600 shadow-[0_4px_14px_rgba(239,68,68,0.35)]'
						: 'from-[#26A768] to-emerald-600 shadow-[0_4px_14px_rgba(84,205,76,0.35)]'}"
				>
					{#if IconComponent}
						<IconComponent class="w-7 h-7 {allInstalled ? 'text-white' : 'text-[#052e16]'}" />
					{/if}
				</div>
				<div class="flex flex-col gap-[0.2rem]">
					<div class="flex items-center gap-[0.6rem]">
						<h1
							class="text-[1.35rem] font-extrabold text-foreground m-0 tracking-[-0.015em] leading-[1.2]"
						>
							{profile.name}
						</h1>
						<span
							class="py-[0.15rem] px-[0.5rem] rounded-full text-[0.58rem] font-extrabold uppercase tracking-[0.08em] leading-[1.6] transition-all duration-300 {allInstalled
								? 'bg-red-500 text-white'
								: 'bg-[#26A768] text-[#052e16]'}"
						>
							{allInstalled ? 'INSTALLED' : 'STABLE'}
						</span>
					</div>
					<p class="m-0 max-w-[22rem] text-[0.8rem] leading-relaxed text-muted-foreground mt-1">
						{profile.description}
					</p>
				</div>
			</div>

			<!-- Right: Actions + Tabs -->
			<div
				class="flex flex-col sm:flex-row items-end sm:items-center gap-4 self-start sm:self-center"
			>
				<!-- Segmented Tabs -->
				<div
					class="flex shrink-0 items-center rounded-lg border border-border/60 bg-muted/40 p-1 shadow-sm"
				>
					<button
						onclick={() => onTabChange('preview')}
						class="flex transition-all cursor-pointer items-center justify-center gap-2 px-4 py-1.5 rounded-md text-sm font-medium {activeTab ===
						'preview'
							? 'bg-background text-[#26A768] shadow-sm font-bold border border-border/30'
							: 'text-muted-foreground hover:text-foreground border border-transparent'}"
					>
						Preview
					</button>
					<button
						onclick={() => onTabChange('package')}
						class="flex transition-all cursor-pointer items-center justify-center gap-2 px-4 py-1.5 rounded-md text-sm font-medium {activeTab ===
						'package'
							? 'bg-background text-[#26A768] shadow-sm font-bold border border-border/30'
							: 'text-muted-foreground hover:text-foreground border border-transparent'}"
					>
						Package
					</button>
				</div>

				<!-- Install / Uninstall buttons from HeroDetail -->
				{#if allInstalled}
					<button
						class="flex items-center gap-1.5 py-[0.55rem] px-5 rounded-[0.65rem] border-none bg-red-500 text-white text-[0.8rem] hover:bg-red-600 shadow-[0_3px_12px_rgba(239,68,68,0.25)] hover:shadow-[0_4px_16px_rgba(239,68,68,0.35)] hover:-translate-y-px font-extrabold transition-all cursor-pointer break-keep whitespace-nowrap"
						onclick={onUninstall}
					>
						<Trash2 class="w-4 h-4" />
						Uninstall Pack
					</button>
				{:else}
					<button
						class="flex items-center gap-1.5 py-[0.55rem] px-5 rounded-[0.65rem] border-none bg-[#26A768] text-[#052e16] hover:bg-[#4bc043] shadow-[0_3px_12px_rgba(84,205,76,0.3)] hover:shadow-[0_4px_16px_rgba(84,205,76,0.4)] hover:-translate-y-px text-[0.8rem] font-extrabold transition-all cursor-pointer break-keep whitespace-nowrap"
						onclick={onInstall}
					>
						<Download class="w-4 h-4" />
						Install Pack
					</button>
				{/if}
			</div>
		</div>

		<hr class="mx-8 border-t border-border/50 max-sm:mx-5" />

		<!-- ── Preview Content ───────────────────────── -->
		{#if activeTab === 'preview'}
			<div class="flex flex-col gap-5 px-8 pt-6 pb-8 max-sm:p-5">
				<div class="space-y-6">
					<div class="flex items-center justify-between">
						<h2 class="text-[0.95rem] font-bold uppercase tracking-widest text-muted-foreground/80">
							Installation Preview
						</h2>
						{#if selectedPackage}
							<button
								onclick={() => (selectedPackage = null)}
								class="cursor-pointer rounded-md border border-border/50 bg-muted/50 px-3 py-1 text-xs font-semibold uppercase tracking-wider text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
							>
								Clear Selection
							</button>
						{/if}
					</div>

					<div class="space-y-8">
						<!-- Official Packages -->
						{#if profile.packages_install.length > 0}
							<div class="space-y-3">
								<h3
									class="flex items-center gap-2 text-[0.8rem] font-extrabold uppercase tracking-widest text-foreground"
								>
									<div class="w-2 h-2 rounded-full bg-emerald-500 shrink-0"></div>
									Official Packages
									<span
										class="ml-1 text-[0.65rem] normal-case font-medium tracking-normal opacity-60 text-muted-foreground"
									>
										— click to see guide
									</span>
								</h3>
								<div class="flex flex-wrap gap-2">
									{#each profile.packages_install as pkg}
										<PackageBadge
											name={pkg}
											variant="official"
											isSelected={selectedPackage === pkg}
											onclick={() => handlePackageClick(pkg)}
										/>
									{/each}
								</div>
							</div>
						{/if}

						<!-- AUR Packages -->
						{#if profile.packages_aur.length > 0}
							<div class="space-y-3">
								<h3
									class="flex items-center gap-2 text-[0.8rem] font-extrabold uppercase tracking-widest text-foreground"
								>
									<div class="w-2 h-2 rounded-full bg-[#26A768] shrink-0"></div>
									AUR Packages
									<span
										class="ml-1 text-[0.65rem] normal-case font-medium tracking-normal opacity-60 text-muted-foreground"
									>
										— click to see guide
									</span>
								</h3>
								<div class="flex flex-wrap gap-2">
									{#each profile.packages_aur as pkg}
										<PackageBadge
											name={pkg}
											variant="aur"
											isSelected={selectedPackage === pkg}
											onclick={() => handlePackageClick(pkg)}
										/>
									{/each}
								</div>
							</div>
						{/if}

						<hr class="border-t border-dashed border-border/60" />

						<!-- ── Service Configuration Guide ─────────────── -->
						<div class="space-y-4">
							<h3 class="text-[0.8rem] font-extrabold uppercase tracking-widest text-foreground">
								Services Configuration & Guides
							</h3>

							{#if selectedPackage && selectedGuide}
								<!-- Show guide for selected package -->
								<div class="animate-in fade-in slide-in-from-bottom-2 duration-300">
									{#if isService}
										<div
											class="mb-4 flex items-center gap-3 rounded-xl border border-[#26A768]/20 bg-[#26A768]/5 px-4 py-3 shadow-[0_2px_10px_rgba(84,205,76,0.05)]"
										>
											<div class="h-2 w-2 shrink-0 rounded-full bg-[#26A768] animate-pulse"></div>
											<span
												class="text-[0.75rem] font-bold uppercase tracking-wider text-[#26A768]/90"
											>
												This package is a service — it will be enabled automatically
											</span>
										</div>
									{/if}
									<ServiceGuideCard
										svc={selectedPackage}
										guide={selectedGuide}
										{copiedCmd}
										onCopy={copyToClipboard}
									/>
								</div>
							{:else if selectedPackage && !selectedGuide}
								<!-- Package selected but no guide available -->
								<div
									class="flex flex-col items-center gap-3 rounded-[1.25rem] border border-border/50 bg-muted/20 py-10 shadow-sm"
								>
									<div
										class="flex h-12 w-12 items-center justify-center rounded-[1rem] bg-muted/60 border border-border/40 shadow-sm"
									>
										<Info class="h-5 w-5 text-muted-foreground/80" />
									</div>
									<div class="text-center">
										<p class="m-0 text-sm font-extrabold text-foreground truncate">
											{selectedPackage}
										</p>
										<p
											class="m-0 mt-1 text-[0.8rem] text-muted-foreground/80 leading-relaxed max-w-sm"
										>
											No specific setup guide available for this package yet.
										</p>
									</div>
								</div>
							{:else}
								<!-- No package selected — show instruction -->
								<div
									class="flex flex-col items-center gap-3 rounded-[1.25rem] border border-dashed border-border/60 bg-muted/10 py-10"
								>
									<div
										class="flex h-12 w-12 items-center justify-center rounded-[1rem] bg-muted/40"
									>
										<Info class="h-5 w-5 text-muted-foreground/50" />
									</div>
									<p class="m-0 text-center text-[0.8rem] font-medium text-muted-foreground/70">
										Click on a package badge above to view its setup guide and commands
									</p>
								</div>
							{/if}
						</div>
					</div>
				</div>
			</div>
		{/if}
	

