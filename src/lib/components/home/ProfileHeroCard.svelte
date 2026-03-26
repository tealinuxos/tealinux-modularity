<!-- <script lang="ts">
	import type { ProfileInfo } from '$lib/commands';
	import { getCategoryIcon, getCategoryColor } from '$lib/utils/category';
	import { CheckCircle2 } from '@lucide/svelte';

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

<div
	class={`absolute top-0 right-0 w-96 h-96 bg-gradient-to-br ${themeColor} opacity-10 blur-3xl -translate-y-1/2 translate-x-1/2 pointer-events-none`}
></div>

<div
	class="relative z-10 flex flex-col md:flex-row justify-between items-start md:items-center gap-6 p-8"
>
	<div class="flex items-start gap-5">
		<div
			class="w-20 h-20 rounded-2xl flex items-center justify-center bg-gradient-to-br from-muted to-muted/50 border border-border shadow-inner"
		>
			<IconComponent class="w-10 h-10 text-foreground" />
		</div>
		<div>
			<div class="flex items-center gap-3 mb-2">
				<h1 class="text-3xl font-bold tracking-tight text-foreground">{profile.name}</h1>
				{#if allInstalled}
					<span
						class="px-2.5 py-1 rounded-full bg-green-500/10 text-green-500 text-[10px] font-bold border border-green-500/20 uppercase tracking-wider"
					>
						Installed
					</span>
				{/if}
			</div>
			<p class="text-muted-foreground max-w-xl leading-relaxed text-sm">
				{profile.description}
			</p>

			<div class="flex items-center gap-4 mt-4 text-[13px] text-muted-foreground">
				<div class="flex items-center gap-1.5 font-medium">
					<CheckCircle2 class="w-4 h-4 text-primary" />
					<span>Curated by {profile.author}</span>
				</div>
				<div class="w-1 h-1 rounded-full bg-border"></div>
				<span class="font-medium">v{profile.version}</span>
			</div>
		</div>
	</div>

	<div class="flex gap-2">
		<div class="bg-muted/50 p-1.5 rounded-xl border border-border flex items-center shadow-inner">
			<button
				onclick={() => onTabChange('preview')}
				class={`px-5 py-2 rounded-lg text-sm font-semibold transition-all duration-200 ${activeTab === 'preview' ? 'bg-background shadow-md text-foreground border-border' : 'text-muted-foreground hover:text-foreground border-transparent'} border`}
			>
				Preview 
			</button>
			<button
				onclick={() => onTabChange('package')}
				class={`px-5 py-2 rounded-lg text-sm font-semibold transition-all duration-200 ${activeTab === 'package' ? 'bg-background shadow-sm text-foreground border-border' : 'text-muted-foreground hover:text-foreground border-transparent'} border`}
			>
				Package
			</button>
		</div>
	</div>
</div>

<hr class="border-border/60 mx-8" /> -->
