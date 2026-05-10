<script lang="ts">
	import { Trash2, Clock, HardDrive } from '@lucide/svelte';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
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
				<span>1.4 GB</span>
			</div>
		</div>
	</Card.Header>

	<Card.Content class="flex-1">
		<p class="text-sm leading-relaxed text-muted-foreground">
			Clear system package cache to free up disk space. This removes downloaded packages from
			previous updates that are no longer needed for daily operation.
		</p>
	</Card.Content>

	<Card.Footer class="flex items-center justify-between border-t h-[70px]">
		<div class="flex items-center gap-1.5 text-xs text-muted-foreground">
			<Clock class="size-3" />
			<span>Last cleaned 3 days ago</span>
		</div>
		<Button variant="destructive" size="sm" class="gap-1.5 font-medium">
			<Trash2 class="size-3.5" />
			Clear Cache
		</Button>
	</Card.Footer>
</Card.Root>
