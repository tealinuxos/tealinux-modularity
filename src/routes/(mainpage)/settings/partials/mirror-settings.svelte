<script lang="ts">
	import { Globe, RefreshCw } from '@lucide/svelte';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Label } from '$lib/components/ui/label';
	import * as Select from '$lib/components/ui/select';

	let refreshing = $state(false);

	async function handleRefresh() {
		refreshing = true;
		await new Promise((r) => setTimeout(r, 1500));
		refreshing = false;
	}
</script>

<Card.Root class="h-full">
	<Card.Header class="pb-3">
		<div class="flex items-center gap-3">
			<div class="flex size-9 shrink-0 items-center justify-center rounded-lg bg-blue-500/10">
				<Globe class="size-4 text-blue-500" />
			</div>
			<div>
				<Card.Title class="text-base">Mirror Settings</Card.Title>
				<Card.Description class="text-xs">Repository source</Card.Description>
			</div>
		</div>
	</Card.Header>

	<Card.Content class="space-y-4">
		<div class="space-y-1.5">
			<Label class="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
				Repository Origin
			</Label>
			<Select.Root type="multiple">
				<Select.Trigger class="w-full">Global (Direct)</Select.Trigger>
				<Select.Content>
					<Select.Item value="global">Global (Direct)</Select.Item>
					<Select.Item value="id">Indonesia</Select.Item>
					<Select.Item value="sg">Singapore</Select.Item>
					<Select.Item value="jp">Japan</Select.Item>
					<Select.Item value="local">Local Mirror</Select.Item>
				</Select.Content>
			</Select.Root>
		</div>

		<Button
			variant="outline"
			class="w-full gap-2 font-medium"
			onclick={handleRefresh}
			disabled={refreshing}
		>
			<RefreshCw class="size-3.5 {refreshing ? 'animate-spin' : ''}" />
			{refreshing ? 'Refreshing…' : 'Refresh Mirror'}
		</Button>
	</Card.Content>
</Card.Root>
