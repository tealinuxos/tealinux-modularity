<script lang="ts">
	import type { Snippet } from 'svelte';
	import { RefreshCcw, TrendingUp } from '@lucide/svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { fetchNews } from '$lib/services/news.service';
	import { useQueryClient } from '@tanstack/svelte-query';

	interface Props {
		children: Snippet;
	}

	const queryClient = useQueryClient();

	const refreshHandler = () => {
		void queryClient.fetchQuery({
			queryKey: ['parsedNews'],
			queryFn: () => fetchNews({ forceRefresh: true })
		});
	};

	let { children }: Props = $props();
</script>

<main class="min-h-screen">
	<div class="flex flex-row items-center justify-between mb-4">
		<div class="flex flex-row items-center gap-x-2">
			<TrendingUp class="text-[#54CD4C] size-8" />
			<h1 class="text-2xl">Latest News & Updates</h1>
		</div>
		<div class="flex flex-row gap-x-2">
			<Button onclick={refreshHandler} variant="outline" title="Refresh combined RSS (Modularitea libs)">
				<RefreshCcw />
			</Button>
		</div>
	</div>
	<p class="text-sm text-muted-foreground mb-4">
		Combined feed (Phoronix, FOSS Linux, ItsFOSS) with 1-hour cache.
	</p>
	<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
		{@render children()}
	</div>
</main>
