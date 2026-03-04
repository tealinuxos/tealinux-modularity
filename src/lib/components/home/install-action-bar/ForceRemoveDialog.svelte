<script lang="ts">
	import { AlertTriangle, Trash2 } from 'lucide-svelte';

	interface Props {
		open: boolean;
		onConfirm: () => void;
		onClose: () => void;
	}

	let { open, onConfirm, onClose }: Props = $props();
</script>

{#if open}
	<!-- Backdrop -->
	<div
		class="fixed inset-0 bg-black/60 backdrop-blur-sm z-50 flex items-center justify-center p-4"
		role="dialog"
		aria-modal="true"
		aria-labelledby="force-dialog-title"
	>
		<div class="bg-card border border-border rounded-2xl shadow-2xl max-w-md w-full p-6 space-y-4">
			<!-- Header -->
			<div class="flex items-start gap-3">
				<div
					class="w-10 h-10 rounded-xl bg-amber-500/10 flex items-center justify-center flex-shrink-0"
				>
					<AlertTriangle class="w-5 h-5 text-amber-500" />
				</div>
				<div>
					<h3 id="force-dialog-title" class="text-lg font-bold text-foreground">Force Remove?</h3>
					<p class="text-sm text-muted-foreground mt-1">
						Some packages couldn't be removed because other software depends on them. Force removing
						will <strong class="text-red-400">skip dependency checks</strong>
						and may cause other applications to stop working.
					</p>
				</div>
			</div>

			<!-- Warning box -->
			<div class="bg-red-500/5 border border-red-500/20 rounded-xl p-3">
				<p class="text-xs text-red-400 font-medium">
					⚠ This action may break other applications that depend on these packages. Only proceed if
					you know what you're doing.
				</p>
			</div>

			<!-- Actions -->
			<div class="flex justify-end gap-3 pt-2">
				<button
					onclick={onClose}
					class="px-5 py-2.5 rounded-xl text-sm font-medium hover:bg-muted transition-colors"
				>
					Cancel
				</button>
				<button
					onclick={() => {
						onClose();
						onConfirm();
					}}
					class="px-5 py-2.5 rounded-xl text-sm font-bold bg-red-500 hover:bg-red-600 text-white
						shadow-lg shadow-red-500/20 active:scale-95 transition-all flex items-center gap-2"
				>
					<Trash2 class="w-4 h-4" />
					Yes, Force Remove
				</button>
			</div>
		</div>
	</div>
{/if}
