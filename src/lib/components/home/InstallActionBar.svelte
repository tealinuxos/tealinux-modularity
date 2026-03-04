<script lang="ts">
	import InstallSizeStats from './install-action-bar/InstallSizeStats.svelte';
	import InstallButtons from './install-action-bar/InstallButtons.svelte';
	import ForceRemoveDialog from './install-action-bar/ForceRemoveDialog.svelte';

	interface Props {
		selectedCount: number;
		uninstallCount: number;
		/** Human-readable download size, e.g. "245.3 MiB" */
		totalSize: string;
		/** Human-readable installed-on-disk size, e.g. "812.1 MiB" */
		totalInstallSize?: string;
		/** True while getPackageSizes is in flight */
		sizeLoading?: boolean;
		estTime: string;
		installState: 'idle' | 'installing' | 'success' | 'error';
		installMessage: string;
		uninstallState: 'idle' | 'uninstalling' | 'success' | 'error';
		uninstallMessage: string;
		hasDependencyError: boolean;
		onInstall: () => void;
		onUninstall: () => void;
		onForceUninstall: () => void;
		onCancel: () => void;
	}

	let {
		selectedCount,
		uninstallCount,
		totalSize,
		totalInstallSize = '',
		sizeLoading = false,
		estTime,
		installState,
		installMessage,
		uninstallState,
		uninstallMessage,
		hasDependencyError,
		onInstall,
		onUninstall,
		onForceUninstall,
		onCancel
	}: Props = $props();

	let isProcessing = $derived(installState === 'installing' || uninstallState === 'uninstalling');
	let showForceDialog = $state(false);
</script>

<!-- Bottom Action Bar -->
<div
	class="w-[calc(100%+2rem)] -mx-4 -mb-4 bg-[#111111] border-t border-white/5 px-8 pt-4 pb-8 flex items-center justify-between mt-auto z-10"
>
	<!-- Left: size statistics -->
	<div class="flex items-center gap-8 min-w-0">
		<div class="flex flex-col gap-1">
			<p class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wide">
				Selection Summary
			</p>
			<div class="flex items-baseline gap-1.5">
				{#if sizeLoading}
					<span class="text-[1.3rem] font-bold tracking-tight text-white leading-none"
						>Fetching...</span
					>
				{:else}
					<span class="text-[1.3rem] font-bold tracking-tight text-white leading-none"
						>{totalSize || '0 B'}</span
					>
				{/if}
				<span class="text-xs text-muted-foreground">/ {selectedCount} Tools Selected</span>
			</div>
		</div>

		<div class="w-px h-10 bg-white/10 flex-shrink-0"></div>

		<div class="flex flex-col gap-1">
			<p class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wide">
				Est. Time
			</p>
			<div class="flex items-baseline gap-1.5">
				<span class="text-[1.3rem] font-bold tracking-tight text-white leading-none"
					>{estTime || '—'}</span
				>
				<span class="text-[10px] text-muted-foreground">(100 Mbps)</span>
			</div>
		</div>
	</div>

	<!-- Right: action buttons -->
	<div class="flex items-center gap-6">
		<button
			onclick={onCancel}
			disabled={isProcessing}
			class="text-sm font-medium text-muted-foreground hover:text-white transition-colors disabled:opacity-40"
		>
			Cancel
		</button>

		{#if hasDependencyError && uninstallCount > 0}
			<button
				onclick={() => (showForceDialog = true)}
				disabled={isProcessing}
				class="px-6 py-2.5 rounded-lg bg-amber-500 hover:bg-amber-600 text-black text-sm font-bold transition-all active:scale-95 disabled:opacity-40"
			>
				Force Remove
			</button>
		{/if}

		{#if uninstallCount > 0 && !hasDependencyError}
			<button
				onclick={onUninstall}
				disabled={isProcessing}
				class="px-6 py-2.5 rounded-lg bg-red-500 hover:bg-red-600 text-white text-sm font-bold transition-all active:scale-95 disabled:opacity-40"
			>
				{#if uninstallState === 'uninstalling'}
					Removing...
				{:else if uninstallState === 'success'}
					Removed
				{:else}
					Uninstall ({uninstallCount})
				{/if}
			</button>
		{/if}

		<button
			onclick={onInstall}
			disabled={selectedCount === 0 || isProcessing}
			class="px-6 py-2.5 rounded-lg bg-[#54CD4C] hover:bg-[#45b03e] text-black text-sm font-bold transition-all active:scale-95 disabled:opacity-40 disabled:cursor-not-allowed"
		>
			{#if installState === 'installing'}
				installing...
			{:else if installState === 'success'}
				done
			{:else}
				start install
			{/if}
		</button>
	</div>
</div>

<!-- Force Remove Confirmation Dialog -->
<ForceRemoveDialog
	open={showForceDialog}
	onConfirm={onForceUninstall}
	onClose={() => (showForceDialog = false)}
/>
