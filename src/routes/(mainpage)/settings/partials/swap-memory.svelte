<script lang="ts">
	import { Database, LoaderCircle } from '@lucide/svelte';
	import * as Card from '$lib/components/ui/card';
	import { Switch } from '$lib/components/ui/switch';
	import { Label } from '$lib/components/ui/label';
	import { commands } from '$lib/commands';
	import { toast } from 'svelte-sonner';
	import { settingsState } from '$lib/state/settings.svelte';
	import { onMount } from 'svelte';

	let updating = $state(false);

	onMount(async () => {
		try {
			const result = await commands.isSwapEnabled();
			if (result.status === 'ok') {
				settingsState.swapEnabled = result.data;
			}
		} catch (err) {
			console.error('Failed to fetch swap status:', err);
		}
	});

	async function handleToggleSwap(checked: boolean) {
		updating = true;
		try {
			const mode = checked ? 'enable' : 'disable';
			const result = await commands.setSwapMode(mode);

			if (result.status === 'ok') {
				settingsState.swapEnabled = checked;
				toast.success(`Swap ${checked ? 'Enabled' : 'Disabled'}`, {
					description: `Virtual memory has been ${checked ? 'activated' : 'deactivated'} successfully.`
				});
			} else {
				toast.error('Failed to change swap status', {
					description: result.error
				});
			}
		} catch (err) {
			toast.error('Unexpected error', {
				description: String(err)
			});
		} finally {
			updating = false;
		}
	}
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
				<div class="flex items-center gap-2">
					<Label for="swap-toggle" class="cursor-pointer text-sm font-medium"
						>Enable Swap File</Label
					>
					{#if updating}
						<LoaderCircle class="size-3 animate-spin text-muted-foreground" />
					{/if}
				</div>
				<p class="text-xs text-muted-foreground">
					{#if settingsState.swapEnabled === null}
						Detecting…
					{:else}
						{settingsState.swapEnabled ? 'Active' : 'Inactive'}
					{/if}
				</p>
			</div>
			<Switch
				id="swap-toggle"
				checked={settingsState.swapEnabled ?? false}
				disabled={updating || settingsState.swapEnabled === null}
				onCheckedChange={handleToggleSwap}
			/>
		</div>
	</Card.Footer>
</Card.Root>
