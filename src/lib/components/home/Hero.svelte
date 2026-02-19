<script lang="ts">
	import { Rocket, Sparkles, Package } from 'lucide-svelte';

	interface Props {
		profileCount: number;
	}

	let { profileCount = 0 }: Props = $props();
</script>

<div
	class="relative w-full bg-gradient-to-br from-green-900 via-green-700 to-emerald-500 rounded-2xl overflow-hidden shadow-xl shadow-green-900/20"
>
	<!-- Decorative Background Elements -->
	<div
		class="absolute inset-0 bg-[radial-gradient(circle_at_30%_50%,rgba(255,255,255,0.08),transparent_60%)]"
	></div>
	<div class="absolute -right-8 -top-8 w-40 h-40 bg-white/5 rounded-full blur-2xl"></div>
	<div class="absolute -left-4 -bottom-4 w-28 h-28 bg-emerald-400/10 rounded-full blur-xl"></div>

	<div class="relative flex items-center justify-between px-8 py-10 md:px-10 md:py-12">
		<!-- Left Content -->
		<div class="z-10 space-y-3">
			<div class="flex items-center gap-2 text-emerald-200/80">
				<Sparkles class="w-4 h-4" />
				<span class="text-xs font-semibold tracking-widest uppercase">ModulariTea</span>
			</div>
			<h1 class="text-3xl md:text-4xl font-bold text-white leading-tight">Profile Catalog</h1>
			<p class="text-emerald-100/70 text-base max-w-md leading-relaxed">
				Browse and install curated tool collections tailored for your workflow.
			</p>
			{#if profileCount > 0}
				<div class="flex items-center gap-2 mt-2 text-emerald-200/60 text-sm">
					<Package class="w-4 h-4" />
					<span>{profileCount} profiles available</span>
				</div>
			{/if}
		</div>

		<!-- Right Decoration -->
		<div class="hidden md:flex items-center gap-3">
			<div
				class="w-16 h-16 bg-white/10 backdrop-blur-sm rounded-2xl flex items-center justify-center transform rotate-6 border border-white/10 shadow-xl transition-transform hover:rotate-0 duration-300"
			>
				<Rocket class="text-white w-8 h-8" />
			</div>
			<div
				class="w-14 h-14 bg-emerald-400/20 backdrop-blur-sm rounded-2xl flex items-center justify-center transform -rotate-3 border border-emerald-300/20 shadow-xl transition-transform hover:rotate-0 duration-300"
			>
				<Package class="text-emerald-200 w-7 h-7" />
			</div>
		</div>
	</div>
</div>
