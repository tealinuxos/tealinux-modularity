<script lang="ts">
	import * as Card from '$lib/components/ui/card/index';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { newsState } from '$lib/state/news.svelte';
	import { fetchRssFeed } from '$lib/utils/news-fetcher';
	import { createQuery } from '@tanstack/svelte-query';

	const newsQuery = createQuery(() => ({
		queryKey: ['rssNews', newsState.provider],
		queryFn: () => fetchRssFeed(newsState.provider)
	}));
	
	console.log(newsQuery)
</script>

{#if newsQuery.isLoading}
	{#each { length: 6 }}
		<Card.Root class="group h-full flex flex-col overflow-hidden bg-card border pt-0">
			<div class="relative w-full aspect-video overflow-hidden border-b">
				<Skeleton class="w-full h-full rounded-none" />
			</div>

			<Card.Header class="p-4 flex flex-col gap-2 flex-1">
				<Skeleton class="h-6 w-full mb-1" />
				<Skeleton class="h-6 w-4/5 mb-3" />

				<Skeleton class="h-4 w-full" />
				<Skeleton class="h-4 w-full" />
				<Skeleton class="h-4 w-2/3" />
			</Card.Header>
		</Card.Root>
	{/each}
{:else if newsQuery.isError}
	<div class="col-span-full py-10 text-center text-destructive">
		<p>Gagal memuat berita: {newsQuery.error.message}</p>
	</div>
{:else if newsQuery.isSuccess}
	{#each newsQuery.data as item (item.id)}
		<a
			href={item.link}
			target="_blank"
			rel="noopener noreferrer"
			class="block h-full focus:outline-none focus-visible:ring-2 focus-visible:ring-primary rounded-xl"
		>
			<Card.Root
				class="group h-full flex flex-col overflow-hidden bg-card transition-all duration-300 hover:shadow-xl hover:-translate-y-1 cursor-pointer border pt-0"
			>
				<div class="relative w-full aspect-video overflow-hidden border-b bg-muted">
					<img
						src={item.thumbnail}
						alt={item.title}
						loading="lazy"
						class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
					/>
				</div>

				<Card.Header class="p-4 flex flex-col gap-1.5 flex-1">
					<Card.Title
						class="text-base md:text-lg font-bold leading-tight line-clamp-2 text-foreground transition-colors group-hover:text-primary"
					>
						{item.title}
					</Card.Title>
					<Card.Description class="text-xs md:text-sm text-muted-foreground line-clamp-3">
						{@html item.description}
					</Card.Description>
				</Card.Header>
			</Card.Root>
		</a>
	{/each}
{/if}
