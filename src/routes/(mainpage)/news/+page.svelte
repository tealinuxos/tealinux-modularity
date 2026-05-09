<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { fetchNews } from '$lib/services/news.service';
	import { createQuery, useQueryClient } from '@tanstack/svelte-query';
	import { decode } from 'html-entities';
	import DOMPurify from 'dompurify';
	import { openUrl } from '@tauri-apps/plugin-opener';
	import { briefErrorMessage } from '$lib/utils/frontend-error-msg';
	import type { ParsedNewsItem } from '$lib/types/news';

	const PLACEHOLDER_THUMB =
		'https://placehold.co/600x400/1e293b/ffffff?text=TealinuxOS&font=montserrat';

	const queryClient = useQueryClient();

	const newsQuery = createQuery(() => ({
		queryKey: ['parsedNews'],
		queryFn: () => fetchNews({ forceRefresh: false })
	}));

	const forceRefresh = () => newsQuery.refetch();

	async function hardRefreshLibs() {
		const r = await fetchNews({ forceRefresh: true });
		queryClient.setQueryData(['parsedNews'], r);
	}

	function stripDescription(item: ParsedNewsItem): string {
		const raw = decode(item.descriptive ?? '');
		const clean = DOMPurify.sanitize(raw, {
			ALLOWED_TAGS: ['b', 'i', 'em', 'strong', 'a', 'p', 'br'],
			ALLOWED_ATTR: ['href', 'target', 'rel']
		});
		return clean.replace(/<[^>]+>/g, ' ').replace(/\s+/g, ' ').trim();
	}

	async function openExternal(url: string, e: MouseEvent) {
		e.preventDefault();
		if (!url || url === '#') return;
		try {
			await openUrl(url);
		} catch {
			window.open(url, '_blank', 'noopener,noreferrer');
		}
	}

	let brokenThumbs = $state<Record<string, boolean>>({});
</script>

{#if newsQuery.isPending}
	{#each Array(6) as _, i (i)}
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
	<div class="col-span-full py-10 text-center flex flex-col gap-3 items-center">
		<p class="text-destructive">Gagal memuat berita.</p>
		<Button variant="outline" onclick={() => forceRefresh()}>Coba lagi</Button>
	</div>
{:else if newsQuery.data?.success === false}
	<div class="col-span-full py-10 text-center flex flex-col gap-2 items-center">
		<p class="text-destructive">
			{briefErrorMessage(newsQuery.data.code, newsQuery.data.error)}
		</p>
		<Button variant="outline" onclick={() => forceRefresh()}>Coba lagi</Button>
	</div>
{:else if newsQuery.data?.success && newsQuery.data.data.length === 0}
	<div class="col-span-full py-16 text-center text-muted-foreground">
		Tidak ada artikel untuk ditampilkan.
		<div class="mt-4">
			<Button variant="outline" onclick={() => hardRefreshLibs()}>
				Refresh paksa
			</Button>
		</div>
	</div>
{:else if newsQuery.data?.success}
	{#each newsQuery.data.data as item (item.url + item.title)}
		<button
			type="button"
			class="block h-full w-full text-left focus:outline-none focus-visible:ring-2 focus-visible:ring-primary rounded-xl"
			onclick={(e) => openExternal(item.url, e)}
		>
			<Card.Root
				class="group h-full flex flex-col overflow-hidden bg-card transition-all duration-300 hover:shadow-xl hover:-translate-y-1 cursor-pointer border pt-0"
			>
				<div class="relative w-full aspect-video overflow-hidden border-b bg-muted">
					<img
						src={brokenThumbs[item.url] ? PLACEHOLDER_THUMB : (item.thumbnail || PLACEHOLDER_THUMB)}
						alt=""
						loading="lazy"
						referrerpolicy="no-referrer"
						class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
						onerror={() => {
							brokenThumbs[item.url] = true;
						}}
					/>
				</div>

				<Card.Header class="p-4 flex flex-col gap-1.5 flex-1">
					<Card.Title
						class="text-base md:text-lg font-bold leading-tight line-clamp-2 text-foreground transition-colors group-hover:text-primary"
					>
						{decode(item.title)}
					</Card.Title>
					<p class="text-xs md:text-sm text-muted-foreground line-clamp-3">{stripDescription(item)}</p>
				</Card.Header>
			</Card.Root>
		</button>
	{/each}
{/if}
