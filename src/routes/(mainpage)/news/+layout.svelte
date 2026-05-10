<script lang="ts">
	import type { Snippet } from 'svelte';
	import { RefreshCcw, TrendingUp } from '@lucide/svelte';
	import * as Select from '$lib/components/ui/select/index';
	import { Separator } from '$lib/components/ui/separator';
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

<div class="space-y-6 pb-6">
	<div class="flex items-center justify-between gap-3">
		<div class="flex items-center gap-3">
			<div class="flex size-10 shrink-0 items-center justify-center rounded-xl bg-primary/10">
				<TrendingUp class="size-5 text-primary" />
			</div>
			<div>
				<h1 class="text-xl font-semibold tracking-tight">Latest News</h1>
				<p class="text-sm text-muted-foreground">Linux news and community updates</p>
			</div>
		</div>

		<div class="flex shrink-0 items-center gap-2">
			<Button onclick={refreshHandler} variant="outline" size="icon">
				<RefreshCcw class="size-4" />
			</Button>
			<Select.Root type="single" bind:value={newsState.provider}>
				<Select.Trigger class="w-36">
					{triggerContent?.label}
				</Select.Trigger>
				<Select.Content>
					<Select.Group>
						<Select.Label>RSS News Provider</Select.Label>
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

	<Separator />

	<!-- News Grid -->
	<div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
		{@render children()}
	</div>
</div>
