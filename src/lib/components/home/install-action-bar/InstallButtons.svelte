<script lang="ts">
	import { Loader2, Check, Download, Trash2, AlertTriangle } from '@lucide/svelte';

	interface Props {
		selectedCount: number;
		uninstallCount: number;
		sizeLoading?: boolean;
		installState: 'idle' | 'installing' | 'success' | 'error';
		installMessage: string;
		uninstallState: 'idle' | 'uninstalling' | 'success' | 'error';
		uninstallMessage: string;
		hasDependencyError: boolean;
		isProcessing: boolean;
		onInstall: () => void;
		onUninstall: () => void;
		onForceRemoveClick: () => void;
		onCancel: () => void;
	}

	let {
		selectedCount,
		uninstallCount,
		sizeLoading = false,
		installState,
		installMessage,
		uninstallState,
		uninstallMessage,
		hasDependencyError,
		isProcessing,
		onInstall,
		onUninstall,
		onForceRemoveClick,
		onCancel
	}: Props = $props();
</script>

<div class="flex items-center gap-2.5 flex-shrink-0">
	<!-- Inline error feedback -->
	{#if installState === 'error'}
		<div class="text-right max-w-[180px]">
			<p class="text-xs font-bold text-red-500 leading-tight">Install Failed</p>
			<p class="text-[10px] text-muted-foreground truncate">{installMessage}</p>
		</div>
	{/if}

	{#if uninstallState === 'error'}
		<div class="text-right max-w-[180px]">
			<p class="text-xs font-bold text-red-500 leading-tight">Uninstall Failed</p>
			<p class="text-[10px] text-muted-foreground truncate">{uninstallMessage}</p>
		</div>
	{/if}

	<!-- Cancel -->
	<button
		onclick={onCancel}
		disabled={isProcessing}
		class="px-5 py-2.5 rounded-xl text-sm font-medium hover:bg-muted transition-colors
			disabled:opacity-40 disabled:cursor-not-allowed"
	>
		Cancel
	</button>

	<!-- Force Remove (only after dependency error) -->
	{#if hasDependencyError && uninstallCount > 0}
		<button
			onclick={onForceRemoveClick}
			disabled={isProcessing}
			class="px-5 py-2.5 rounded-xl text-sm font-bold flex items-center gap-2 transition-all
				bg-amber-500 hover:bg-amber-600 text-black shadow-lg shadow-amber-500/20
				active:scale-95 disabled:opacity-40"
		>
			<AlertTriangle class="w-4 h-4" />
			Force Remove
		</button>
	{/if}

	<!-- Uninstall (when packages marked for removal, no dependency error) -->
	{#if uninstallCount > 0 && !hasDependencyError}
		<button
			onclick={onUninstall}
			disabled={isProcessing}
			class={`px-5 py-2.5 rounded-xl text-sm font-bold flex items-center gap-2 transition-all
				${
					uninstallState === 'success'
						? 'bg-green-500 hover:bg-green-600 text-white shadow-lg shadow-green-500/20'
						: isProcessing
							? 'bg-red-400 text-white cursor-wait'
							: 'bg-red-500 hover:bg-red-600 text-white shadow-lg shadow-red-500/20 active:scale-95 disabled:opacity-40'
				}`}
		>
			{#if uninstallState === 'uninstalling'}
				<Loader2 class="w-4 h-4 animate-spin" />
				Removing…
			{:else if uninstallState === 'success'}
				<Check class="w-4 h-4" />
				Removed
			{:else}
				<Trash2 class="w-4 h-4" />
				Uninstall ({uninstallCount})
			{/if}
		</button>
	{/if}

	<!-- Install -->
	<button
		onclick={onInstall}
		disabled={selectedCount === 0 || isProcessing}
		class={`px-6 py-2.5 rounded-xl text-sm font-bold flex items-center gap-2 transition-all
			${
				installState === 'success'
					? 'bg-green-500 hover:bg-green-600 text-white shadow-lg shadow-green-500/20'
					: selectedCount === 0 || isProcessing
						? 'bg-muted text-muted-foreground cursor-not-allowed'
						: sizeLoading
							? 'bg-[#26A768] text-[#052e16] shadow-lg shadow-green-500/20 animate-pulse'
							: 'bg-[#26A768] hover:bg-[#4bc043] text-[#052e16] shadow-lg shadow-green-500/20 active:scale-95'
			}`}
	>
		{#if installState === 'installing'}
			<Loader2 class="w-4 h-4 animate-spin" />
			Installing…
		{:else if installState === 'success'}
			<Check class="w-4 h-4" />
			Done
		{:else}
			<Download class="w-4 h-4" />
			{sizeLoading && selectedCount > 0 ? 'Calculating…' : 'Start Install'}
		{/if}
	</button>
</div>
