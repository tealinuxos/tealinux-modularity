<script lang="ts">
	import { Gauge, Zap, Cpu, Battery, LoaderCircle } from '@lucide/svelte';
	import * as Card from '$lib/components/ui/card';
	import { commands, type CpuProfile } from '$lib/commands';
	import { toast } from 'svelte-sonner';
	import { settingsState } from '$lib/state/settings.svelte';
	import { onMount } from 'svelte';

	let updating = $state(false);
	let loaded = $state(false);

	onMount(async () => {
		try {
			const result = await commands.getCpuGovernorState();
			if (result.status === 'ok') {
				settingsState.cpuGovernor = result.data;
			}
		} catch (err) {
			console.error('Failed to fetch CPU governor:', err);
		} finally {
			loaded = true;
		}
	});

	const governors: {
		value: CpuProfile;
		label: string;
		sub: string;
		Icon: typeof Cpu;
		iconClass: string;
	}[] = [
		{
			value: 'powersave',
			label: 'Powersave',
			sub: 'Saves battery',
			Icon: Battery,
			iconClass: 'text-yellow-500'
		},
		{
			value: 'performance',
			label: 'Performance',
			sub: 'Max speed',
			Icon: Zap,
			iconClass: 'text-primary-foreground'
		},
		{
			value: 'ondemand',
			label: 'Ondemand',
			sub: 'Balanced',
			Icon: Cpu,
			iconClass: 'text-blue-500'
		}
	];

	async function handleSetProfile(profile: CpuProfile) {
		if (settingsState.cpuGovernor === profile) return;

		updating = true;
		try {
			const result = await commands.setCpuProfile(profile);
			if (result.status === 'ok') {
				settingsState.cpuGovernor = profile;
				toast.success('CPU Profile Updated', {
					description: `System is now running in ${profile} mode.`
				});
			} else {
				toast.error('Failed to set CPU profile', {
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

<Card.Root>
	<div class="flex flex-col gap-5 p-5 sm:flex-row sm:items-center">
		<div class="flex flex-1 items-center gap-3">
			<div class="flex size-9 shrink-0 items-center justify-center rounded-lg bg-orange-500/10">
				<Gauge class="size-4 text-orange-500" />
			</div>
			<div>
				<p class="text-sm font-semibold">CPU Performance</p>
				<p class="mt-0.5 max-w-sm text-xs leading-relaxed text-muted-foreground">
					{#if !loaded}
						Detecting current governor…
					{:else if settingsState.cpuGovernor === null}
						Custom governor detected — pick a supported profile to override
					{:else}
						Adjust the frequency scaling governor to balance energy efficiency and raw speed.
					{/if}
				</p>
			</div>
		</div>

		<div class="grid grid-cols-3 gap-2 sm:flex sm:shrink-0 sm:items-stretch">
			{#each governors as { value, label, sub, Icon, iconClass } (value)}
				{@const isActive = settingsState.cpuGovernor === value}
				{@const isLoading = !loaded}
				<button
					onclick={() => handleSetProfile(value)}
					disabled={updating || isLoading}
					class="flex min-w-25 flex-col items-center gap-1.5 rounded-xl border px-4 py-3 transition-all
						{isActive
						? 'border-primary bg-primary text-primary-foreground shadow-sm'
						: 'border-border bg-muted/30 text-foreground hover:bg-muted/60'}
						{updating || isLoading ? 'opacity-50 cursor-not-allowed' : ''}"
				>
					{#if updating && isActive}
						<LoaderCircle class="size-4 animate-spin text-primary-foreground" />
					{:else}
						<Icon class="size-4 {isActive ? 'text-primary-foreground' : iconClass}" />
					{/if}
					<span class="text-xs font-semibold leading-none">{label}</span>
					<span
						class="text-[10px] font-medium uppercase tracking-wide
							{isActive ? 'text-primary-foreground/70' : 'text-muted-foreground'}"
					>
						{isActive ? 'Active' : sub}
					</span>
				</button>
			{/each}
		</div>
	</div>
</Card.Root>
