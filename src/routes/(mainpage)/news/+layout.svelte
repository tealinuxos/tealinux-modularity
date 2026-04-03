<script lang="ts">
	import type { Snippet } from 'svelte';
	import { RefreshCcw, TrendingUp } from '@lucide/svelte';
	import * as Select from '$lib/components/ui/select/index';
	import Button from '$lib/components/ui/button/button.svelte';
	import { newsState } from '$lib/state/news.svelte';
	import type { RSS_URL } from '$lib/utils/news-fetcher';
	import { useQueryClient } from '@tanstack/svelte-query';

	interface Props {
		children: Snippet;
	}

	const providers: { label: string; url: RSS_URL }[] = [
		{ label: 'ItsFOSS', url: 'https://itsfoss.com/rss/' },
		{ label: 'Phoronix', url: 'https://www.phoronix.com/rss.php' },
		{ label: 'FOSS Linux', url: 'https://fosslinux.com/feed' }
	];

	const triggerContent = $derived(providers.find((f) => f.url === newsState.provider));

	const queryClient = useQueryClient();

	const refreshHandler = () => {
		queryClient.invalidateQueries({
			queryKey: ['rssNews', newsState.provider]
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
			<Button onclick={refreshHandler} variant="outline">
				<RefreshCcw />
			</Button>
			<Select.Root type="single" bind:value={newsState.provider}>
				<Select.Trigger class="w-45">
					{triggerContent?.label}
				</Select.Trigger>
				<Select.Content>
					<Select.Group>
						<Select.Label>Select RSS News Provider</Select.Label>
						{#each providers as item (item.url)}
							<Select.Item
								value={item.url}
								label={item.label}
								disabled={newsState.provider === item.url}
							>
								{item.label}
							</Select.Item>
						{/each}
					</Select.Group>
				</Select.Content>
			</Select.Root>
		</div>
	</div>
	<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
		{@render children()}
	</div>
</main>
