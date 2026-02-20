<script lang="ts">
	import { Package, Check } from 'lucide-svelte';

	interface Props {
		packages: string[];
		selectedPackages: Set<string>;
		installedPackages: Set<string>;
		onToggle: (pkg: string) => void;
	}

	let { packages, selectedPackages, installedPackages, onToggle }: Props = $props();
</script>

<div class="space-y-4">
	<div class="bg-card border border-border rounded-xl overflow-hidden divide-y divide-border">
		{#each packages as pkg}
			{@const installed = installedPackages.has(pkg)}
			{@const selected = selectedPackages.has(pkg)}

			<button
				onclick={() => !installed && onToggle(pkg)}
				class={`w-full flex items-center justify-between p-4 hover:bg-muted/50 transition-colors ${installed ? 'opacity-75 cursor-default' : 'cursor-pointer'}`}
			>
				<div class="flex items-center gap-4">
					<div
						class={`w-10 h-10 rounded-lg flex items-center justify-center ${installed ? 'bg-green-500/10 text-green-500' : 'bg-muted text-muted-foreground'}`}
					>
						{#if installed}
							<Check class="w-5 h-5" />
						{:else}
							<Package class="w-5 h-5" />
						{/if}
					</div>
					<div class="text-left">
						<div class="font-medium text-foreground flex items-center gap-2">
							{pkg}
							{#if installed}
								<span
									class="text-[10px] uppercase font-bold text-green-500 bg-green-500/10 px-1.5 py-0.5 rounded"
									>Installed</span
								>
							{/if}
						</div>
						<div class="text-xs text-muted-foreground">Official Repository</div>
					</div>
				</div>

				{#if !installed}
					<div
						class={`w-5 h-5 rounded border flex items-center justify-center transition-colors ${selected ? 'bg-primary border-primary text-primary-foreground' : 'border-input bg-background'}`}
					>
						{#if selected}
							<Check class="w-3.5 h-3.5" />
						{/if}
					</div>
				{/if}
			</button>
		{/each}
	</div>
</div>
