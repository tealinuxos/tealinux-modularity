<script lang="ts">
	import { Package, Check, Trash2 } from 'lucide-svelte';

	interface Props {
		packages: string[];
		selectedPackages: Set<string>;
		installedPackages: Set<string>;
		selectedUninstallPackages: Set<string>;
		onToggle: (pkg: string) => void;
		onToggleUninstall: (pkg: string) => void;
	}

	let {
		packages,
		selectedPackages,
		installedPackages,
		selectedUninstallPackages,
		onToggle,
		onToggleUninstall
	}: Props = $props();
</script>

<div class="space-y-6 mt-4">
	<div class="flex items-center gap-3">
		<div
			class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center border border-primary/20"
		>
			<Package class="w-4 h-4 text-primary" />
		</div>
		<h2 class="text-xl font-bold">Manage Packages</h2>
	</div>

	<div class="border border-border rounded-2xl overflow-hidden divide-y divide-border bg-muted/20">
		{#each packages as pkg}
			{@const installed = installedPackages.has(pkg)}
			{@const selected = selectedPackages.has(pkg)}
			{@const markedForRemoval = selectedUninstallPackages.has(pkg)}

			<button
				onclick={() => {
					if (installed) {
						onToggleUninstall(pkg);
					} else {
						onToggle(pkg);
					}
				}}
				class={`w-full flex items-center justify-between p-5 hover:bg-muted/50 transition-colors cursor-pointer ${markedForRemoval ? 'bg-red-500/5' : ''}`}
			>
				<div class="flex items-center gap-4">
					<div
						class={`w-10 h-10 rounded-xl flex items-center justify-center border transition-all duration-200 ${
							markedForRemoval
								? 'bg-red-500/10 text-red-500 border-red-500/20'
								: installed
									? 'bg-green-500/10 text-green-500 border-green-500/20'
									: 'bg-background text-muted-foreground border-border shadow-sm'
						}`}
					>
						{#if markedForRemoval}
							<Trash2 class="w-5 h-5" />
						{:else if installed}
							<Check class="w-5 h-5" />
						{:else}
							<Package class="w-5 h-5" />
						{/if}
					</div>
					<div class="text-left">
						<div class="font-bold text-foreground flex items-center gap-3">
							<span class="text-base">{pkg}</span>
							{#if markedForRemoval}
								<span
									class="text-[10px] uppercase font-bold text-red-500 bg-red-500/10 px-2 py-0.5 rounded-full border border-red-500/20 tracking-wider"
									>Marked for Removal</span
								>
							{:else if installed}
								<span
									class="text-[10px] uppercase font-bold text-green-500 bg-green-500/10 px-2 py-0.5 rounded-full border border-green-500/20 tracking-wider"
									>Installed</span
								>
							{/if}
						</div>
						<div class="text-xs text-muted-foreground font-medium mt-0.5">Official Repository</div>
					</div>
				</div>

				{#if markedForRemoval}
					<!-- Red checkbox for marked uninstall -->
					<div
						class="w-6 h-6 rounded-lg border flex items-center justify-center transition-all duration-200 bg-red-500 border-red-500 text-white shadow-lg shadow-red-500/20 scale-105"
					>
						<Trash2 class="w-4 h-4" strokeWidth={3} />
					</div>
				{:else if !installed}
					<!-- Install selection checkbox -->
					<div
						class={`w-6 h-6 rounded-lg border flex items-center justify-center transition-all duration-200 ${selected ? 'bg-primary border-primary text-primary-foreground shadow-lg shadow-primary/20 scale-105' : 'border-input bg-background shadow-inner'}`}
					>
						{#if selected}
							<Check class="w-4 h-4" strokeWidth={3} />
						{/if}
					</div>
				{/if}
			</button>
		{/each}
	</div>
</div>
