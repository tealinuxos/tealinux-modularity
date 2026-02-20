<script lang="ts">
	import type { ProfileInfo } from '$lib/commands';
	import { getCategoryIcon, getCategoryColor } from '$lib/utils/category';
	import { CheckCircle2, Package } from 'lucide-svelte';

	interface Props {
		profile: ProfileInfo;
		allInstalled: boolean;
		activeTab: 'preview' | 'package';
		onTabChange: (tab: 'preview' | 'package') => void;
	}

	let { profile, allInstalled, activeTab, onTabChange }: Props = $props();

	let IconComponent = $derived(getCategoryIcon(profile.category));
	let themeColor = $derived(getCategoryColor(profile.category));
</script>

<div class="relative overflow-hidden rounded-3xl bg-card border border-border p-8 shadow-xl">
	<!-- Background glow -->
	<div
		class={`absolute top-0 right-0 w-96 h-96 bg-gradient-to-br ${themeColor} opacity-10 blur-3xl -translate-y-1/2 translate-x-1/2 pointer-events-none`}
	></div>

	<div
		class="relative z-10 flex flex-col md:flex-row justify-between items-start md:items-center gap-6"
	>
		<div class="flex items-start gap-5">
			<div
				class={`w-20 h-20 rounded-2xl flex items-center justify-center bg-gradient-to-br from-muted to-muted/50 border border-border shadow-inner`}
			>
				<IconComponent class="w-10 h-10 text-foreground" />
			</div>
			<div>
				<div class="flex items-center gap-3 mb-2">
					<h1 class="text-3xl font-bold tracking-tight">{profile.name}</h1>
					{#if allInstalled}
						<span
							class="px-2 py-0.5 rounded-full bg-green-500/10 text-green-500 text-xs font-medium border border-green-500/20"
							>Installed</span
						>
					{/if}
				</div>
				<p class="text-muted-foreground max-w-xl leading-relaxed">
					{profile.description}
				</p>

				<div class="flex items-center gap-4 mt-4 text-sm text-muted-foreground">
					<div class="flex items-center gap-1.5">
						<CheckCircle2 class="w-4 h-4 text-primary" />
						<span>Curated by {profile.author}</span>
					</div>
					<div class="w-1 h-1 rounded-full bg-border"></div>
					<span>v{profile.version}</span>
				</div>
			</div>
		</div>

		<div class="flex gap-2">
			<div class="bg-muted/50 p-1 rounded-lg border border-border flex">
				<button
					onclick={() => onTabChange('preview')}
					class={`px-4 py-1.5 rounded-md text-sm font-medium transition-all ${activeTab === 'preview' ? 'bg-background shadow-sm text-foreground' : 'text-muted-foreground hover:text-foreground'}`}
				>
					Preview
				</button>
				<button
					onclick={() => onTabChange('package')}
					class={`px-4 py-1.5 rounded-md text-sm font-medium transition-all ${activeTab === 'package' ? 'bg-background shadow-sm text-foreground' : 'text-muted-foreground hover:text-foreground'}`}
				>
					Package
				</button>
			</div>
		</div>
	</div>
</div>
