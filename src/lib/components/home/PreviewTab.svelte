<script lang="ts">
	import type { ProfileInfo } from '$lib/commands';
	import PackageBadge from './PackageBadge.svelte';
	import ServiceGuideCard from './ServiceGuideCard.svelte';
	import { getCategoryIcon } from '$lib/utils/category';
	import packageGuidesRaw from '$lib/data/packageGuides.json';
	import { Download, Trash2, Info } from '@lucide/svelte';
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

<div class="flex flex-col">
	<div
		class="flex items-start justify-between gap-6 px-8 pt-7 pb-5 max-sm:flex-col max-sm:px-5 max-sm:pt-5 max-sm:pb-4"
	>
		<!-- Left: icon + identity -->
		<div class="flex items-center gap-4">
			<div
				class="w-[3.25rem] h-[3.25rem] rounded-[0.875rem] shrink-0 flex items-center justify-center transition-all duration-300 bg-gradient-to-br {allInstalled
					? 'from-red-500 to-red-600 shadow-[0_4px_14px_rgba(239,68,68,0.35)]'
					: 'from-green-500 to-green-600 shadow-[0_4px_14px_rgba(34,197,94,0.35)]'}"
			>
				{#if IconComponent}
					<IconComponent class="w-7 h-7 text-white" />
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
						class="py-[0.1rem] px-[0.5rem] rounded-full text-[0.58rem] font-extrabold uppercase tracking-[0.07em] leading-[1.6] transition-all duration-300 {allInstalled
							? 'bg-red-500 text-white'
							: 'bg-green-500 text-[#052e16]'}"
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
		<div class="flex flex-col sm:flex-row items-end sm:items-center gap-4">
			<div class="flex shrink-0 items-center rounded-lg border border-border bg-muted/40 p-0.5">
				<button
					onclick={() => onTabChange('preview')}
					class={`cursor-pointer rounded-md px-4 py-1.5 text-sm font-medium transition-all ${activeTab === 'preview' ? 'bg-background text-[#54CD4C] shadow-sm' : 'text-muted-foreground hover:text-foreground'}`}
				>
					Preview
				</button>
				<button
					onclick={() => onTabChange('package')}
					class={`cursor-pointer rounded-md px-4 py-1.5 text-sm font-medium transition-all ${activeTab === 'package' ? 'bg-background text-[#54CD4C] shadow-sm' : 'text-muted-foreground hover:text-foreground'}`}
				>
					Package
				</button>
			</div>

			<!-- Install / Uninstall buttons from HeroDetail -->
			{#if allInstalled}
				<button
					class="flex items-center gap-2 py-[0.6rem] px-[1.35rem] rounded-[0.65rem] border-none bg-red-500 text-white text-[0.82rem] font-bold cursor-pointer transition-all duration-200 whitespace-nowrap shadow-[0_3px_12px_rgba(239,68,68,0.3)] hover:bg-red-600 hover:-translate-y-[1px] hover:shadow-[0_6px_20px_rgba(239,68,68,0.4)] active:translate-y-0 w-full sm:w-auto justify-center shrink-0"
					onclick={onUninstall}
				>
					<Trash2 class="w-4 h-4" />
					Uninstall Pack
				</button>
			{:else}
				<button
					class="flex items-center gap-2 py-[0.6rem] px-[1.35rem] rounded-[0.65rem] border-none bg-green-500 text-[#052e16] text-[0.82rem] font-bold cursor-pointer transition-all duration-200 whitespace-nowrap shadow-[0_3px_12px_rgba(34,197,94,0.3)] hover:bg-green-600 hover:-translate-y-[1px] hover:shadow-[0_6px_20px_rgba(34,197,94,0.4)] active:translate-y-0 w-full sm:w-auto justify-center shrink-0"
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
			<div class="space-y-6 rounded-2xl border border-border bg-card p-6">
				<div class="flex items-center justify-between">
					<h2 class="text-lg font-semibold">Installation Preview</h2>
					{#if selectedPackage}
						<button
							onclick={() => (selectedPackage = null)}
							class="cursor-pointer rounded-md border border-border bg-muted/50 px-3 py-1 text-xs text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
						>
							Clear Selection
						</button>
					{/if}
				</div>

				<div class="space-y-6">
					<!-- Official Packages -->
					{#if profile.packages_install.length > 0}
						<div class="space-y-3">
							<h3 class="text-sm font-medium uppercase tracking-wider text-muted-foreground">
								Official Packages
								<span class="ml-1 text-xs normal-case tracking-normal opacity-60">
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
							<h3 class="text-sm font-medium uppercase tracking-wider text-muted-foreground">
								AUR Packages
								<span class="ml-1 text-xs normal-case tracking-normal opacity-60">
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

					<!-- ── Service Configuration Guide ─────────────── -->
					<div class="space-y-3">
						<h3 class="text-sm font-medium uppercase tracking-wider text-muted-foreground">
							Services Configuration & Guides
						</h3>

						{#if selectedPackage && selectedGuide}
							<!-- Show guide for selected package -->
							<div class="animate-in fade-in slide-in-from-bottom-2 duration-200">
								{#if isService}
									<div
										class="mb-3 flex items-center gap-2 rounded-lg border border-[#54CD4C]/20 bg-[#54CD4C]/5 px-3 py-2"
									>
										<div class="h-2 w-2 shrink-0 rounded-full bg-[#54CD4C] animate-pulse"></div>
										<span class="text-xs font-medium text-[#54CD4C]">
											This package is a service — it will be enabled automatically on install
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
								class="flex flex-col items-center gap-3 rounded-xl border border-border bg-muted/30 py-8"
							>
								<div class="flex h-10 w-10 items-center justify-center rounded-full bg-muted">
									<Info class="h-5 w-5 text-muted-foreground" />
								</div>
								<div class="text-center">
									<p class="m-0 text-sm font-semibold text-foreground capitalize">
										{selectedPackage}
									</p>
									<p class="m-0 mt-1 text-xs text-muted-foreground">
										No specific guide available for this package yet.
									</p>
								</div>
							</div>
						{:else}
							<!-- No package selected — show instruction -->
							<div
								class="flex flex-col items-center gap-3 rounded-xl border border-dashed border-border/60 bg-muted/20 py-8"
							>
								<div class="flex h-10 w-10 items-center justify-center rounded-full bg-muted/60">
									<Info class="h-5 w-5 text-muted-foreground/60" />
								</div>
								<p class="m-0 text-center text-sm text-muted-foreground">
									Click on a package above to view its setup guide and commands
								</p>
							</div>
						{/if}
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>
