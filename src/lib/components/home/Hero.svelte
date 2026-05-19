<script lang="ts">
	import { Package } from '@lucide/svelte';

	interface Props {
		profileCount: number;
	}

	let { profileCount = 0 }: Props = $props();
</script>

<div class="relative w-full overflow-hidden rounded-[var(--radius-xl)]" style="background: #26A768;">

	<!-- Leaf accents -->
	<svg class="pointer-events-none absolute -right-2 -top-4 opacity-[0.13]" style="width:200px;height:160px" viewBox="0 0 200 160" fill="none" aria-hidden="true">
		<path d="M160 10 C180 30, 195 70, 170 100 C150 125, 120 130, 100 120 C130 100, 145 70, 130 40 Z" fill="white"/>
		<path d="M130 40 L100 120" stroke="white" stroke-width="1.5" stroke-linecap="round"/>
		<path d="M130 40 C125 55, 118 70, 100 120" stroke="white" stroke-width="0.5" stroke-linecap="round" opacity="0.6"/>
		<path d="M148 55 C140 68, 128 80, 110 108" stroke="white" stroke-width="0.5" stroke-linecap="round" opacity="0.5"/>
		<path d="M158 72 C150 83, 137 95, 120 115" stroke="white" stroke-width="0.5" stroke-linecap="round" opacity="0.4"/>
	</svg>

	<svg class="pointer-events-none absolute bottom-[-22px] right-12 opacity-[0.09]" style="width:130px;height:110px;transform:rotate(25deg)" viewBox="0 0 130 110" fill="none" aria-hidden="true">
		<path d="M80 5 C100 20, 120 55, 100 80 C82 100, 55 102, 40 90 C65 72, 78 48, 65 22 Z" fill="white"/>
		<path d="M65 22 L40 90" stroke="white" stroke-width="1.2" stroke-linecap="round"/>
		<path d="M80 35 C72 50, 60 66, 48 84" stroke="white" stroke-width="0.5" stroke-linecap="round" opacity="0.5"/>
		<path d="M90 55 C82 67, 70 78, 58 92" stroke="white" stroke-width="0.5" stroke-linecap="round" opacity="0.4"/>
	</svg>

	<svg class="pointer-events-none absolute -top-3 left-[42%] opacity-[0.07]" style="width:80px;height:70px;transform:rotate(-15deg)" viewBox="0 0 80 70" fill="none" aria-hidden="true">
		<path d="M50 4 C64 14, 75 38, 60 55 C48 68, 28 68, 18 58 C35 46, 46 30, 36 14 Z" fill="white"/>
		<path d="M36 14 L18 58" stroke="white" stroke-width="1" stroke-linecap="round"/>
		<path d="M50 25 C44 36, 36 47, 26 60" stroke="white" stroke-width="0.5" stroke-linecap="round" opacity="0.5"/>
	</svg>

	<svg class="pointer-events-none absolute -bottom-2 left-2 opacity-[0.08]" style="width:90px;height:80px;transform:rotate(40deg) scaleX(-1)" viewBox="0 0 90 80" fill="none" aria-hidden="true">
		<path d="M55 5 C70 18, 82 45, 65 63 C52 76, 30 76, 20 65 C38 52, 50 35, 40 16 Z" fill="white"/>
		<path d="M40 16 L20 65" stroke="white" stroke-width="1" stroke-linecap="round"/>
		<path d="M55 27 C48 40, 38 53, 28 68" stroke="white" stroke-width="0.5" stroke-linecap="round" opacity="0.5"/>
	</svg>

	<!-- Content -->
	<div class="relative z-10 flex items-center justify-between gap-6 px-9 py-8 md:px-10">

		<!-- Left -->
		<div class="flex flex-col gap-2">
			<div class="flex items-center gap-2">
				<div class="h-[5px] w-[5px] rounded-full bg-white/50"></div>
				<span class="text-[11px] font-medium uppercase tracking-[0.07em] text-white/65">ModulariTea</span>
			</div>

			<h1 class="text-[1.6rem] font-semibold leading-tight tracking-tight text-white md:text-[1.75rem]">
				Profile catalog
			</h1>

			<p class="max-w-sm text-[0.82rem] leading-relaxed text-white/70">
				Browse and install curated tool collections tailored for your workflow.
			</p>

			{#if profileCount > 0}
				<div class="mt-1 flex items-center gap-1.5 text-[0.75rem] text-white/60">
					<Package class="h-[0.9rem] w-[0.9rem]" />
					<span>{profileCount} profiles available</span>
				</div>
			{/if}
		</div>

		<!-- Right: app icons -->
		<div class="hidden shrink-0 items-center gap-2.5 md:flex">
			<div class="flex h-[52px] w-[52px] items-center justify-center rounded-[13px] border border-white/15 bg-black/[0.18] transition-transform duration-200 hover:-translate-y-0.5">
				<svg class="h-6 w-6 text-white/85" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
					<polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/>
				</svg>
			</div>
			<div class="flex h-[52px] w-[52px] items-center justify-center rounded-[13px] border border-white/15 bg-black/[0.18] transition-transform duration-200 hover:-translate-y-0.5">
				<svg class="h-6 w-6 text-white/85" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
					<rect x="2" y="3" width="20" height="14" rx="2"/><line x1="8" y1="21" x2="16" y2="21"/><line x1="12" y1="17" x2="12" y2="21"/>
				</svg>
			</div>
			<div class="flex h-[52px] w-[52px] items-center justify-center rounded-[13px] border border-white/15 bg-black/[0.18] transition-transform duration-200 hover:-translate-y-0.5">
				<svg class="h-6 w-6 text-white/85" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
					<path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/>
				</svg>
			</div>
		</div>

	</div>
</div>