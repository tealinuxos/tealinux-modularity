<script lang="ts">
	import { Globe, RefreshCw } from '@lucide/svelte';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Label } from '$lib/components/ui/label';
	import * as Select from '$lib/components/ui/select';
	import { commands } from '$lib/commands';
	import { toast } from 'svelte-sonner';

	type MIRROR_VALUE = {
		label: string;
		country: string;
	};

	const mirrors: Record<string, MIRROR_VALUE> = {
		global: { label: 'Global (Direct)', country: 'US,DE,NL,SG' },
		id: { label: 'Indonesia', country: 'ID' },
		sg: { label: 'Singapore', country: 'SG' },
		jp: { label: 'Japan', country: 'JP' },
		de: { label: 'Germany', country: 'DE' },
		us: { label: 'United States', country: 'US' }
	};

	let selected = $state('id');
	let refreshing = $state(false);

	const selectedCountry = $derived(mirrors[selected]?.country);

	async function handleRefresh() {
		refreshing = true;
		try {
			const [result] = await Promise.all([commands.refreshMirror(selectedCountry)]);

			if (result.status === 'ok') {
				toast.success('Mirror refreshed', {
					description: `Now using ${mirrors[selected].label} mirrors`
				});
			} else {
				toast.error('Failed to refresh the mirror list.', { description: 'Administrator privileges are required.' });
			}
		} catch (err) {
			toast.error('Unexpected error', { description: String(err) });
		} finally {
			refreshing = false;
		}
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
			<Select.Root type="single" bind:value={selected}>
				<Select.Trigger class="w-full">
					{mirrors[selected]?.label ?? 'Select mirror'}
				</Select.Trigger>
				<Select.Content>
					{#each Object.entries(mirrors) as [key, mirror] (key)}
						<Select.Item value={key}>{mirror.label}</Select.Item>
					{/each}
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
