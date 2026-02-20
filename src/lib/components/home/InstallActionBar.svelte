<script lang="ts">
	import { Loader2, Check, Download } from 'lucide-svelte';

	interface Props {
		selectedCount: number;
		totalSize: string;
		estTime: string;
		installState: 'idle' | 'installing' | 'success' | 'error';
		installMessage: string;
		onInstall: () => void;
		onCancel: () => void;
	}

	let {
		selectedCount,
		totalSize,
		estTime,
		installState,
		installMessage,
		onInstall,
		onCancel
	}: Props = $props();
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
					<span class="text-sm text-muted-foreground">/ {selectedCount} Tools Selected</span>
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

			<button
				onclick={onCancel}
				class="px-6 py-3 rounded-xl text-sm font-medium hover:bg-muted transition-colors"
			>
				Cancel
			</button>

			<button
				onclick={onInstall}
				disabled={selectedCount === 0 || installState === 'installing'}
				class={`px-6 py-3 rounded-xl text-sm font-bold flex items-center gap-2 transition-all
					${
						installState === 'success'
							? 'bg-green-500 hover:bg-green-600 text-white shadow-lg shadow-green-500/20'
							: selectedCount === 0
								? 'bg-muted text-muted-foreground cursor-not-allowed'
								: 'bg-[#54CD4C] hover:bg-[#45b03e] text-black shadow-lg shadow-green-500/20 active:scale-95'
					}`}
			>
				{#if installState === 'installing'}
					<Loader2 class="w-4 h-4 animate-spin" />
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
