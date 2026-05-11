<script lang="ts">
	import { Database } from '@lucide/svelte';
	import * as Card from '$lib/components/ui/card';
	import { Switch } from '$lib/components/ui/switch';
	import { Label } from '$lib/components/ui/label';

	let enabled = $state(false);
</script>

<Card.Root class="flex h-full flex-col">
	<Card.Header class="pb-3">
		<div class="flex items-center gap-3">
			<div class="flex size-9 shrink-0 items-center justify-center rounded-lg bg-primary/10">
				<Database class="size-4 text-primary" />
			</div>
			<div>
				<Card.Title class="text-base">Swap Memory</Card.Title>
				<Card.Description class="text-xs">Virtual memory extension</Card.Description>
			</div>
		</div>
	</Card.Header>

	<Card.Content class="flex-1">
		<p class="text-sm leading-relaxed text-muted-foreground">
			Enable virtual memory on your storage device to prevent system crashes during heavy
			multitasking.
		</p>
	</Card.Content>

	<Card.Footer class="border-t pt-4">
		<div class="flex w-full items-center justify-between">
			<div class="space-y-0.5">
				<Label for="swap-toggle" class="cursor-pointer text-sm font-medium">Enable Swap File</Label>
				<p class="text-xs text-muted-foreground">
					{enabled ? 'Active — 2 GB allocated' : 'Inactive'}
				</p>
			</div>
			<Switch id="swap-toggle" bind:checked={enabled} />
		</div>
	</Card.Footer>
</Card.Root>
