<script lang="ts">
	import { commands, type Computer, type Audio, type Display } from '$lib/commands';
	import { onMount } from 'svelte';
	import { Cpu } from '@lucide/svelte';
	import * as Accordion from '$lib/components/ui/accordion/';
	import { Separator } from '$lib/components/ui/separator';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import prettyBytes from 'pretty-bytes';

	type AsyncState<T> =
		| { status: 'loading' }
		| { status: 'success'; data: T }
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		| { status: 'error'; error: any };

	let computerState = $state<AsyncState<Computer>>({ status: 'loading' });
	let displayState = $state<AsyncState<Display>>({ status: 'loading' });
	let audioState = $state<AsyncState<Audio>>({ status: 'loading' });

	const formatLabel = (key: string) => {
		return key
			.replace(/_/g, ' ')
			.replace(/([A-Z])/g, ' $1')
			.replace(/^./, (str) => str.toUpperCase())
			.trim();
	};

	onMount(async () => {
		const results = await Promise.allSettled([
			commands.computerInfo(),
			commands.displayInfo(),
			commands.audioInfo()
		]);

		function mapResult<T>(result: PromiseSettledResult<T>): AsyncState<T> {
			if (result.status === 'fulfilled') {
				return { status: 'success', data: result.value };
			} else {
				return { status: 'error', error: result.reason };
			}
		}

		const [resComputer, resDisplay, resAudio] = results;

		computerState = mapResult(resComputer as PromiseSettledResult<Computer>);
		displayState = mapResult(resDisplay as PromiseSettledResult<Display>);
		audioState = mapResult(resAudio as PromiseSettledResult<Audio>);
	});
</script>

{#snippet loadingSkeleton()}
	<div class="space-y-4 py-2">
		{#each { length: 3 }, index (index)}
			<div class="flex flex-row items-start gap-4 py-2 border-b border-border/40 last:border-0">
				<div class="w-1/3">
					<Skeleton class="h-4 w-24 bg-muted" />
				</div>
				<div class="w-2/3 space-y-2">
					<Skeleton class="h-4 w-full bg-muted/80" />
					<Skeleton class="h-4 w-2/3 bg-muted/50" />
				</div>
			</div>
		{/each}
	</div>
{/snippet}

{#snippet infoRow(key: string, value: string | bigint | string[])}
	<div class="flex flex-row py-3 border-b border-border/40 last:border-0 items-start text-sm">
		<div class="w-1/3 font-medium text-muted-foreground wrap-break-word pr-4 shrink-0">
			{formatLabel(key)}
		</div>

		<div class="w-2/3 text-foreground font-medium min-w-0">
			{#if Array.isArray(value)}
				{#if value.length > 1}
					<ul class="list-disc list-outside ml-4 space-y-1 marker:text-muted-foreground">
						{#each value as item, index (index)}
							<li class="wrap-break-word pl-1">{item}</li>
						{/each}
					</ul>
				{:else if value.length === 1}
					{value}
				{:else}
					<span class="text-muted-foreground italic text-xs">- Empty -</span>
				{/if}
			{:else if key === 'memory'}
				{prettyBytes(value as bigint, { binary: true, minimumFractionDigits: 2 })}
			{:else}
				{value}
			{/if}
		</div>
	</div>
{/snippet}

{#snippet stateRenderer(state: AsyncState<Computer | Display | Audio>)}
	{#if state.status === 'loading'}
		{@render loadingSkeleton()}
	{:else if state.status === 'error'}
		<div
			class="p-3 bg-destructive/10 text-destructive text-sm rounded-md border border-destructive/20"
		>
			Error: {state.error?.message || 'Failed to load data'}
		</div>
	{:else if state.status === 'success'}
		<div class="flex flex-col">
			{#each Object.entries(state.data) as [key, value] (key)}
				{@render infoRow(key, value)}
			{/each}
		</div>
	{/if}
{/snippet}

<div class="space-y-6 pb-6">
	<div class="flex items-center gap-3">
		<div class="flex size-10 shrink-0 items-center justify-center rounded-xl bg-primary/10">
			<Cpu class="size-5 text-[#26A768]" />
		</div>
		<div>
			<h1 class="text-xl font-semibold tracking-tight">System Information</h1>
			<p class="text-sm text-muted-foreground">Hardware and system specifications</p>
		</div>
	</div>

	<Separator />

	<Accordion.Root
		type="multiple"
		class="bg-sidebar rounded-lg w-full shadow-sm flex flex-col"
		value={['computer', 'display']}
	>
		<Accordion.Item class="px-3 border-b" value="computer">
			<Accordion.Trigger class="group">Computer</Accordion.Trigger>
			<Accordion.Content>
				{@render stateRenderer(computerState)}
			</Accordion.Content>
		</Accordion.Item>

		<Accordion.Item class="px-3 border-b" value="display">
			<Accordion.Trigger class="group">Display</Accordion.Trigger>
			<Accordion.Content>
				{@render stateRenderer(displayState)}
			</Accordion.Content>
		</Accordion.Item>

		<Accordion.Item class="px-3 border-b-0" value="audio">
			<Accordion.Trigger class="group">Audio</Accordion.Trigger>
			<Accordion.Content>
				{@render stateRenderer(audioState)}
			</Accordion.Content>
		</Accordion.Item>
	</Accordion.Root>
</div>
