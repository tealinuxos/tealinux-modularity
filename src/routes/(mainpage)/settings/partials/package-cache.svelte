<script lang="ts">
	import { Trash2, Clock, HardDrive, LoaderCircle } from '@lucide/svelte';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { commands } from '$lib/commands';
	import { toast } from 'svelte-sonner';
	import { onMount } from 'svelte';
	import { settingsState } from '$lib/state/settings.svelte';

	let clearing = $state(false);
	let cacheSize = $state<string>('0 B');

	async function fetchCacheSize() {
		try {
			const result = await commands.getCacheSize();
			if (result.status === 'ok') {
				cacheSize = formatBytes(Number(result.data));
			}
		} catch (err) {
			console.error('Failed to fetch cache size:', err);
		}
	}

	function formatBytes(bytes: number, decimals = 2) {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const dm = decimals < 0 ? 0 : decimals;
		const sizes = ['B', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
	}

	function formatLastCleaned(timestamp: number | null) {
		if (!timestamp) return 'Never cleaned';
		const diff = Date.now() - timestamp;
		const minutes = Math.floor(diff / 60000);
		if (minutes < 1) return 'Just now';
		if (minutes < 60) return `${minutes}m ago`;
		const hours = Math.floor(minutes / 60);
		if (hours < 24) return `${hours}h ago`;
		const days = Math.floor(hours / 24);
		return `${days}d ago`;
	}

	onMount(() => {
		fetchCacheSize();
	});

	async function handleClear() {
		clearing = true;
		try {
			const result = await commands.cleanCache();
			if (result.status === 'ok') {
				settingsState.lastCacheCleaned = Date.now();
				toast.success('Package cache cleared', {
					description: 'Disk space has been freed successfully'
				});
				await fetchCacheSize();
			} else {
				toast.error('Failed to clear cache', { description: result.error });
			}
		} catch (err) {
			toast.error('Unexpected error', { description: String(err) });
		} finally {
			clearing = false;
		}
	}
</script>

<Card.Root class="flex h-full flex-col">
	<Card.Header class="pb-3">
		<div class="flex items-start justify-between gap-4">
			<div class="flex items-center gap-3">
				<div class="flex size-9 shrink-0 items-center justify-center rounded-lg bg-red-500/10">
					<Trash2 class="size-4 text-red-500" />
				</div>
				<div>
					<Card.Title class="text-base">Package Cache</Card.Title>
					<Card.Description class="text-xs">Pacman download store</Card.Description>
				</div>
			</div>
			<div
				class="flex shrink-0 items-center gap-1.5 rounded-full bg-muted px-2.5 py-1 text-xs font-medium text-muted-foreground"
			>
				<HardDrive class="size-3" />
				<span>{cacheSize}</span>
			</div>
		</div>
	</Card.Header>

	<Card.Content class="flex-1">
		<p class="text-sm leading-relaxed text-muted-foreground">
			Clear system package cache to free up disk space. This removes downloaded packages from
			previous updates that are no longer needed for daily operation.
		</p>
	</Card.Content>

	<Card.Footer class="flex h-17.5 items-center justify-between border-t">
		<div class="flex items-center gap-1.5 text-xs text-muted-foreground">
			<Clock class="size-3" />
			<span>Last cleaned {formatLastCleaned(settingsState.lastCacheCleaned)}</span>
		</div>
		<Button
			variant="destructive"
			size="sm"
			class="gap-1.5 font-medium"
			onclick={handleClear}
			disabled={clearing}
		>
			{#if clearing}
				<LoaderCircle class="size-3.5 animate-spin" />
				Clearing…
			{:else}
				<Trash2 class="size-3.5" />
				Clear Cache
			{/if}
		</Button>
	</Card.Footer>
</Card.Root>
