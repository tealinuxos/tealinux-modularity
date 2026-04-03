<script lang="ts">
	import { LoaderCircle, Check, Download, Trash2, TriangleAlert } from '@lucide/svelte';

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

<div class="border-t border-border bg-card/80 backdrop-blur p-4">
	<div class="max-w-5xl mx-auto flex items-center justify-between">
		<div class="flex items-center gap-8">
			<div>
				<p class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider mb-0.5">
					Selection Summary
				</p>
				<div class="flex items-baseline gap-1.5">
					<span class="text-xl font-bold text-foreground">{totalSize}</span>
					<span class="text-sm text-muted-foreground">
						/ {selectedCount} to Install
						{#if uninstallCount > 0}
							<span class="text-red-400">• {uninstallCount} to Remove</span>
						{/if}
					</span>
				</div>
			</div>
			<div class="pl-8 border-l border-border h-8 flex flex-col justify-center">
				<p class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider mb-0.5">
					Est. Time
				</p>
				<div class="flex items-baseline gap-1">
					<span class="text-base font-bold text-foreground">{estTime}</span>
					<span class="text-xs text-muted-foreground">(100 Mbps)</span>
				</div>
			</div>
		</div>

		<div class="flex items-center gap-3">
			{#if installState === 'error'}
				<div class="text-right mr-2">
					<p class="text-xs font-bold text-red-500">Installation Failed</p>
					<p class="text-[10px] text-muted-foreground max-w-[200px] truncate">
						{installMessage}
					</p>
				</div>
			{/if}

			{#if uninstallState === 'error'}
				<div class="text-right mr-2">
					<p class="text-xs font-bold text-red-500">Uninstall Failed</p>
					<p class="text-[10px] text-muted-foreground max-w-[200px] truncate">
						{uninstallMessage}
					</p>
				</div>
			{/if}

			<button
				onclick={onCancel}
				disabled={isProcessing}
				class="px-6 py-3 rounded-xl text-sm font-medium hover:bg-muted transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
			>
				Cancel
			</button>

			<!-- Force Remove Button (appears after dependency error) -->
			{#if hasDependencyError && uninstallCount > 0}
				<button
					onclick={() => (showForceConfirmDialog = true)}
					disabled={isProcessing}
					class="px-6 py-3 rounded-xl text-sm font-bold flex items-center gap-2 transition-all
						bg-amber-500 hover:bg-amber-600 text-black shadow-lg shadow-amber-500/20 active:scale-95"
				>
					<TriangleAlert class="w-4 h-4" />
					Force Remove
				</button>
			{/if}

			<!-- Uninstall Button (only when packages are marked for removal) -->
			{#if uninstallCount > 0 && !hasDependencyError}
				<button
					onclick={onUninstall}
					disabled={isProcessing}
					class={`px-6 py-3 rounded-xl text-sm font-bold flex items-center gap-2 transition-all
						${
							uninstallState === 'success'
								? 'bg-green-500 hover:bg-green-600 text-white shadow-lg shadow-green-500/20'
								: isProcessing
									? 'bg-red-400 text-white cursor-wait'
									: 'bg-red-500 hover:bg-red-600 text-white shadow-lg shadow-red-500/20 active:scale-95'
						}`}
				>
					{#if uninstallState === 'uninstalling'}
						<LoaderCircle class="w-4 h-4 animate-spin" />
						Removing...
					{:else if uninstallState === 'success'}
						<Check class="w-4 h-4" />
						Removed
					{:else}
						<Trash2 class="w-4 h-4" />
						Uninstall ({uninstallCount})
					{/if}
				</button>
			{/if}

			<!-- Install Button -->
			<button
				onclick={onInstall}
				disabled={selectedCount === 0 || isProcessing}
				class={`px-6 py-3 rounded-xl text-sm font-bold flex items-center gap-2 transition-all
					${
						installState === 'success'
							? 'bg-green-500 hover:bg-green-600 text-white shadow-lg shadow-green-500/20'
							: selectedCount === 0 || isProcessing
								? 'bg-muted text-muted-foreground cursor-not-allowed'
								: 'bg-[#54CD4C] hover:bg-[#45b03e] text-black shadow-lg shadow-green-500/20 active:scale-95'
					}`}
			>
				{#if installState === 'installing'}
					<LoaderCircle class="w-4 h-4 animate-spin" />
					Installing...
				{:else if installState === 'success'}
					<Check class="w-4 h-4" />
					Done
				{:else}
					<Download class="w-4 h-4" />
					Start Install
				{/if}
			</button>
		</div>
	</div>
</div>

<!-- Force Remove Confirmation Dialog -->
{#if showForceConfirmDialog}
	<div class="fixed inset-0 bg-black/60 backdrop-blur-sm z-50 flex items-center justify-center p-4">
		<div class="bg-card border border-border rounded-2xl shadow-2xl max-w-md w-full p-6 space-y-4">
			<div class="flex items-start gap-3">
				<div
					class="w-10 h-10 rounded-xl bg-amber-500/10 flex items-center justify-center flex-shrink-0"
				>
					<TriangleAlert class="w-5 h-5 text-amber-500" />
				</div>
				<div>
					<h3 class="text-lg font-bold text-foreground">Force Remove?</h3>
					<p class="text-sm text-muted-foreground mt-1">
						Some packages couldn't be removed because other software depends on them. Force removing
						will <strong class="text-red-400">skip dependency checks</strong>
						and may cause other applications to stop working.
					</p>
				</div>
			</div>

			<div class="bg-red-500/5 border border-red-500/20 rounded-xl p-3">
				<p class="text-xs text-red-400 font-medium">
					⚠ This action may break other applications that depend on these packages. Only proceed if
					you know what you're doing.
				</p>
			</div>

			<div class="flex justify-end gap-3 pt-2">
				<button
					onclick={() => (showForceConfirmDialog = false)}
					class="px-5 py-2.5 rounded-xl text-sm font-medium hover:bg-muted transition-colors"
				>
					Cancel
				</button>
				<button
					onclick={() => {
						showForceConfirmDialog = false;
						onForceUninstall();
					}}
					class="px-5 py-2.5 rounded-xl text-sm font-bold bg-red-500 hover:bg-red-600 text-white shadow-lg shadow-red-500/20 active:scale-95 transition-all flex items-center gap-2"
				>
					<Trash2 class="w-4 h-4" />
					Yes, Force Remove
				</button>
			</div>
		</div>
	</div>
{/if}
