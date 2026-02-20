<script lang="ts">
	import type { ProfileInfo } from '$lib/commands';
	import { Package, Download, Terminal } from 'lucide-svelte';

	interface Props {
		profile: ProfileInfo;
	}

	let { profile }: Props = $props();
</script>

<div class="rounded-2xl bg-card border border-border p-6 space-y-6">
	<div class="flex justify-between items-center">
		<h2 class="text-lg font-semibold">Installation Preview</h2>
	</div>

	<div class="space-y-6">
		<!-- Official Packages -->
		<div class="space-y-3">
			<h3 class="text-sm font-medium text-muted-foreground uppercase tracking-wider">
				Official Packages
			</h3>
			<div class="flex flex-wrap gap-2">
				{#each profile.packages_install as pkg}
					<div
						class="flex items-center gap-2 px-3 py-2 rounded-lg bg-muted/50 border border-border"
					>
						<Package class="w-4 h-4 text-muted-foreground" />
						<span class="font-mono text-sm">{pkg}</span>
					</div>
				{/each}
			</div>
		</div>

		<!-- AUR Packages -->
		{#if profile.packages_aur.length > 0}
			<div class="space-y-3">
				<h3 class="text-sm font-medium text-muted-foreground uppercase tracking-wider">
					AUR Packages
				</h3>
				<div class="flex flex-wrap gap-2">
					{#each profile.packages_aur as pkg}
						<div
							class="flex items-center gap-2 px-3 py-2 rounded-lg bg-card border border-border border-l-2 border-l-amber-500/50"
						>
							<Download class="w-4 h-4 text-amber-500" />
							<span class="font-mono text-sm">{pkg}</span>
						</div>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Post-install Commands / Services -->
		{#if profile.services_enable.length > 0}
			<div class="space-y-3">
				<h3 class="text-sm font-medium text-muted-foreground uppercase tracking-wider">
					Services configuration
				</h3>
				<div class="flex flex-wrap gap-2">
					{#each profile.services_enable as svc}
						<div
							class="flex items-center gap-2 px-3 py-2 rounded-lg bg-card border border-border border-l-2 border-l-blue-500/50"
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
