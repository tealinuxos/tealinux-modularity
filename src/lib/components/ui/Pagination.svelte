<script lang="ts">
	import { ChevronLeft, ChevronRight, MoreHorizontal } from '@lucide/svelte';

	interface Props {
		currentPage: number;
		totalPages: number;
	}

	let { currentPage = $bindable(), totalPages }: Props = $props();

	function nextPage() {
		if (currentPage < totalPages) currentPage++;
	}

	function prevPage() {
		if (currentPage > 1) currentPage--;
	}

	let pages = $derived.by(() => {
		const items: (number | string)[] = [];
		const maxVisible = 7;

		if (totalPages <= maxVisible) {
			for (let i = 1; i <= totalPages; i++) items.push(i);
		} else {
			items.push(1);
			let start = Math.max(2, currentPage - 1);
			let end = Math.min(totalPages - 1, currentPage + 1);

			if (currentPage <= 3) {
				end = 4;
			}
			if (currentPage >= totalPages - 2) {
				start = totalPages - 3;
			}

			if (start > 2) {
				items.push('...');
			}

			for (let i = start; i <= end; i++) {
				items.push(i);
			}

			if (end < totalPages - 1) {
				items.push('...');
			}
			items.push(totalPages);
		}
		return items;
	});
</script>

{#if totalPages > 1}
	<div class="flex items-center justify-between pt-2">
		<p class="text-xs text-muted-foreground m-0 hidden sm:block">
			Page <span class="font-bold text-foreground">{currentPage}</span> of
			<span class="font-bold text-foreground">{totalPages}</span>
		</p>
		<div class="flex items-center gap-1 sm:gap-2 mx-auto sm:mx-0">
			<button
				onclick={prevPage}
				disabled={currentPage === 1}
				class="flex cursor-pointer items-center justify-center p-2 rounded-lg border border-border/50 bg-background text-foreground hover:bg-muted disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
			>
				<ChevronLeft class="w-4 h-4" />
			</button>

			<div class="flex items-center gap-1">
				{#each pages as p}
					{#if p === '...'}
						<div class="flex items-center justify-center w-8 h-8 text-muted-foreground">
							<MoreHorizontal class="w-4 h-4" />
						</div>
					{:else}
						<button
							onclick={() => (currentPage = p as number)}
							class="flex cursor-pointer items-center justify-center min-w-8 h-8 px-2 rounded-lg border border-border/50 text-sm font-medium transition-colors {currentPage ===
							p
								? 'bg-[#26A768] text-white border-[#26A768]'
								: 'bg-background text-foreground hover:bg-muted'}"
						>
							{p}
						</button>
					{/if}
				{/each}
			</div>

			<button
				onclick={nextPage}
				disabled={currentPage === totalPages}
				class="flex cursor-pointer items-center justify-center p-2 rounded-lg border border-border/50 bg-background text-foreground hover:bg-muted disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
			>
				<ChevronRight class="w-4 h-4" />
			</button>
		</div>
	</div>
{/if}
