<script lang="ts">
	import { Search, X, Loader2 } from '@lucide/svelte';

	interface Props {
		onsearch: (query: string) => void;
		loading?: boolean;
	}

	let { onsearch, loading = false }: Props = $props();

	let query = $state('');
	let timer: number | null = $state(null);

	function handleInput() {
		if (timer !== null) {
			clearTimeout(timer);
			timer = null;
		}
		const q = query;
		timer = window.setTimeout(() => {
			timer = null;
			if (q.trim().length >= 2) {
				onsearch(q.trim());
			}
		}, 400);
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && query.trim().length >= 2) {
			if (timer !== null) {
				clearTimeout(timer);
				timer = null;
			}
			onsearch(query.trim());
		}
	}

	function clearQuery() {
		query = '';
		onsearch('');
	}
</script>

<div class="relative w-full">
	<div
		class="flex items-center gap-3 px-4 py-3 rounded-xl border border-border/60 bg-muted/40
               focus-within:ring-4 focus-within:ring-[#26A768]/15 focus-within:border-[#26A768]/50
               transition-all duration-300 shadow-sm"
	>
		{#if loading}
			<Loader2 class="w-5 h-5 text-muted-foreground animate-spin shrink-0" />
		{:else}
			<Search class="w-5 h-5 text-muted-foreground shrink-0" />
		{/if}

		<input
			id="aur-search-input"
			type="text"
			bind:value={query}
			oninput={handleInput}
			onkeydown={handleKeydown}
			placeholder="Search AUR packages... (min 2 characters)"
			class="flex-1 bg-transparent text-foreground placeholder:text-muted-foreground
                   text-sm outline-none"
		/>

		{#if query.length > 0}
			<button
				onclick={clearQuery}
				class="p-1 rounded-md hover:bg-accent transition-colors"
				aria-label="Clear search"
			>
				<X class="w-4 h-4 text-muted-foreground" />
			</button>
		{/if}
	</div>
</div>
