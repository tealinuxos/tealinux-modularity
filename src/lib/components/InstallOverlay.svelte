<script lang="ts">
	/**
	 * InstallOverlay.svelte — Global persistent install progress overlay.
	 *
	 * Rendered in the root layout so it survives ALL navigation.
	 * Reads from the global installStore — never stores local state for the install.
	 */
	import { installStore } from '$lib/stores/install.svelte';
	import {
		Loader2,
		CheckCircle2,
		XCircle,
		Terminal,
		X,
		ChevronDown,
		ChevronUp,
		Package,
		Download,
		Settings,
		Database
	} from '@lucide/svelte';
	import { fly, fade, slide } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { cancelInstall } from '$lib/services/installManager';

	// Local UI state only — does NOT affect install
	let expanded = $state(true);
	let logContainer = $state<HTMLDivElement | null>(null);

	// Auto-scroll logs to bottom on new entries
	$effect(() => {
		const logs = installStore.activeInstall?.logs;
		if (logs && logContainer) {
			// Defer so DOM has time to update
			setTimeout(() => {
				if (logContainer) {
					logContainer.scrollTop = logContainer.scrollHeight;
				}
			}, 50);
		}
	});

	const task = $derived(installStore.activeInstall);
	const visible = $derived(task !== null);
	const isActive = $derived(task?.phase === 'installing');
	const isSuccess = $derived(task?.phase === 'success');
	const isError = $derived(task?.phase === 'error' || task?.phase === 'cancelled');

	const stepIcon = $derived(() => {
		switch (task?.step) {
			case 'db-update':
				return Database;
			case 'installing':
				return Download;
			case 'services':
				return Settings;
			default:
				return Package;
		}
	});

	const progressColor = $derived(
		isSuccess
			? 'bg-emerald-500'
			: isError
				? 'bg-red-500'
				: 'bg-[#26A768]'
	);

	const borderColor = $derived(
		isSuccess
			? 'border-emerald-500/30'
			: isError
				? 'border-red-500/30'
				: 'border-[#26A768]/30'
	);

	async function handleCancel() {
		if (!task || !isActive) return;
		await cancelInstall();
	}

	function formatLogLine(line: string, stream: string) {
		if (stream === 'stderr') return { text: line, cls: 'text-amber-400/90' };
		if (line.startsWith('✓')) return { text: line, cls: 'text-emerald-400' };
		if (line.startsWith('✗') || line.startsWith('Error') || line.toLowerCase().includes('error'))
			return { text: line, cls: 'text-red-400' };
		if (line.startsWith('⚠') || line.toLowerCase().includes('warning'))
			return { text: line, cls: 'text-amber-400' };
		if (stream === 'system') return { text: line, cls: 'text-[#26A768]/90' };
		return { text: line, cls: 'text-zinc-300/80' };
	}
</script>

{#if visible && task}
	<div
		transition:fly={{ y: 80, duration: 400, easing: cubicOut }}
		class="fixed bottom-5 right-5 z-[9999] w-[400px] max-w-[calc(100vw-2.5rem)]
               rounded-2xl border {borderColor} bg-[#0f0f0f]/95 backdrop-blur-xl
               shadow-2xl shadow-black/60 overflow-hidden"
		role="status"
		aria-live="polite"
		aria-label="Install progress"
	>
		<!-- ── Header ─────────────────────────────────────────────────────────── -->
		<div class="flex items-center gap-3 px-4 pt-3.5 pb-3">
			<!-- Status icon -->
			<div
				class="flex-shrink-0 w-8 h-8 rounded-xl flex items-center justify-center
                       {isSuccess
					? 'bg-emerald-500/15'
					: isError
						? 'bg-red-500/15'
						: 'bg-[#26A768]/15'}"
			>
				{#if isActive}
					<Loader2 class="w-4 h-4 text-[#26A768] animate-spin" />
				{:else if isSuccess}
					<CheckCircle2 class="w-4 h-4 text-emerald-500" />
				{:else}
					<XCircle class="w-4 h-4 text-red-500" />
				{/if}
			</div>

			<!-- Title + message -->
			<div class="flex-1 min-w-0">
				<p class="text-[0.8rem] font-bold text-white leading-tight truncate">
					{task.label}
				</p>
				<p class="text-[0.7rem] text-zinc-400 leading-tight mt-0.5 truncate">
					{task.message}
				</p>
			</div>

			<!-- Controls -->
			<div class="flex items-center gap-1 flex-shrink-0">
				<!-- Expand/collapse logs -->
				<button
					onclick={() => (expanded = !expanded)}
					class="w-7 h-7 rounded-lg flex items-center justify-center
                           text-zinc-500 hover:text-zinc-200 hover:bg-white/5
                           transition-all duration-150"
					title={expanded ? 'Collapse logs' : 'Expand logs'}
				>
					{#if expanded}
						<ChevronDown class="w-3.5 h-3.5" />
					{:else}
						<ChevronUp class="w-3.5 h-3.5" />
					{/if}
				</button>

				<!-- Cancel / close -->
				{#if isActive}
					<button
						onclick={handleCancel}
						class="w-7 h-7 rounded-lg flex items-center justify-center
                               text-zinc-500 hover:text-red-400 hover:bg-red-500/10
                               transition-all duration-150"
						title="Cancel install"
					>
						<X class="w-3.5 h-3.5" />
					</button>
				{:else}
					<button
						onclick={() => installStore.resetInstall()}
						class="w-7 h-7 rounded-lg flex items-center justify-center
                               text-zinc-500 hover:text-zinc-200 hover:bg-white/5
                               transition-all duration-150"
						title="Dismiss"
					>
						<X class="w-3.5 h-3.5" />
					</button>
				{/if}
			</div>
		</div>

		<!-- ── Progress bar ───────────────────────────────────────────────────── -->
		<div class="mx-4 mb-3">
			<div class="flex items-center justify-between mb-1.5">
				<div class="flex items-center gap-1.5">
					<!-- Step indicator -->
					{#if isActive}
						{@const StepIcon = stepIcon()}
						<StepIcon class="w-3 h-3 text-zinc-500" />
						<span class="text-[0.65rem] text-zinc-500 uppercase tracking-wider font-bold">
							{task.step === 'db-update'
								? 'Updating DB'
								: task.step === 'installing'
									? 'Installing'
									: task.step === 'services'
										? 'Enabling services'
										: 'Preparing'}
						</span>
					{/if}
				</div>
				<span class="text-[0.65rem] font-bold {isSuccess ? 'text-emerald-400' : isError ? 'text-red-400' : 'text-zinc-400'}">
					{task.progress}%
				</span>
			</div>

			<div class="h-1.5 rounded-full bg-zinc-800 overflow-hidden">
				<div
					class="h-full rounded-full transition-all duration-500 ease-out {progressColor}
                       {isActive ? 'relative overflow-hidden' : ''}"
					style="width: {task.progress}%"
				>
					{#if isActive}
						<!-- Shimmer animation -->
						<div
							class="absolute inset-0 -translate-x-full animate-[shimmer_1.8s_infinite]
                               bg-gradient-to-r from-transparent via-white/20 to-transparent"
						></div>
					{/if}
				</div>
			</div>
		</div>

		<!-- ── Package count badges ───────────────────────────────────────────── -->
		<div class="mx-4 mb-2 flex items-center gap-2">
			<span class="text-[0.6rem] px-2 py-0.5 rounded-full bg-zinc-800 text-zinc-400 font-medium">
				{task.packages.length} package{task.packages.length !== 1 ? 's' : ''}
			</span>
			{#if task.services.length > 0}
				<span
					class="text-[0.6rem] px-2 py-0.5 rounded-full bg-zinc-800 text-zinc-400 font-medium"
				>
					{task.services.length} service{task.services.length !== 1 ? 's' : ''}
				</span>
			{/if}
			<span class="text-[0.6rem] text-zinc-600 ml-auto font-mono">
				{task.id}
			</span>
		</div>

		<!-- ── Terminal log panel ─────────────────────────────────────────────── -->
		{#if expanded}
			<div transition:slide={{ duration: 250, easing: cubicOut }}>
				<div class="mx-4 mb-3 rounded-xl bg-black/50 border border-zinc-800/80 overflow-hidden">
					<div
						class="flex items-center gap-2 px-3 py-1.5 border-b border-zinc-800/60
                               bg-zinc-900/60"
					>
						<Terminal class="w-3 h-3 text-zinc-500" />
						<span class="text-[0.6rem] uppercase tracking-widest text-zinc-500 font-bold"
							>Terminal Output</span
						>
						<span class="ml-auto text-[0.6rem] text-zinc-600">{task.logs.length} lines</span>
					</div>

					<div
						bind:this={logContainer}
						class="h-[140px] overflow-y-auto p-2.5 font-mono text-[0.67rem] leading-[1.6]
                               scrollbar-thin scrollbar-track-transparent scrollbar-thumb-zinc-700"
					>
						{#if task.logs.length === 0}
							<p class="text-zinc-600 italic">Waiting for output…</p>
						{:else}
							{#each task.logs as entry (entry.ts + entry.line)}
								{@const { cls } = formatLogLine(entry.line, entry.stream)}
								<div class="{cls} break-all whitespace-pre-wrap">
									{entry.line}
								</div>
							{/each}
						{/if}
					</div>
				</div>
			</div>
		{/if}
	</div>
{/if}
