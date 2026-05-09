';<script lang="ts">
	import { AlertTriangle, Cpu, Droplets, LoaderCircle, Settings, Shuffle, Trash2, Zap } from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import * as Select from '$lib/components/ui/select/index';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { cn } from '$lib/utils';
	import {
		changeDns,
		cleanPackageCache,
		listMirrorCountries,
		readCpuGovernor,
		readDnsSummary,
		refreshMirror,
		setCpuProfile,
		swapEnabledFromConfig,
		toggleSwap
	} from '$lib/services/settings.service';
	import type { CpuProfile, DnsProvider } from '$lib/types/settings';
	import { sanitizeCountryChoice } from '$lib/utils/safe-args';
	import { createMutation, createQuery } from '@tanstack/svelte-query';

	const dnsOptions: DnsProvider[] = ['cloudflare', 'google', 'quad9'];

	let mirrorCountryPick = $state('');
	let mirrorSearchInput = $state('');
	let dnsPick = $state<DnsProvider>('cloudflare');
	let cpuPick = $state<CpuProfile>('ondemand');

	const countriesQuery = createQuery(() => ({
		queryKey: ['mirrorCountries'],
		queryFn: async () => {
			const r = await listMirrorCountries();
			if (!r.success) throw new Error(r.error);
			return [...r.data].sort((a, b) => a.localeCompare(b));
		},
		staleTime: Infinity,
		refetchOnWindowFocus: false
	}));

	let countriesInitialized = false;
	$effect(() => {
		if (countriesQuery.isSuccess && countriesQuery.data?.length && !countriesInitialized) {
			const indonesia = sanitizeCountryChoice(countriesQuery.data, 'Indonesia');
			mirrorCountryPick = indonesia ?? countriesQuery.data[0];
			countriesInitialized = true;
		}
	});

	const dnsSummaryQuery = createQuery(() => ({
		queryKey: ['dnsSummary'],
		queryFn: async () => {
			const r = await readDnsSummary();
			if (!r.success) throw new Error(r.error);
			return r.data;
		},
		refetchOnWindowFocus: false
	}));

	const swapQuery = createQuery(() => ({
		queryKey: ['swapZramCfg'],
		queryFn: async () => {
			const r = await swapEnabledFromConfig();
			if (!r.success) throw new Error(r.error);
			return r.data;
		},
		refetchOnWindowFocus: false
	}));

	const governorQuery = createQuery(() => ({
		queryKey: ['cpuGovernor'],
		queryFn: async () => {
			const r = await readCpuGovernor();
			if (!r.success) throw new Error(r.error);
			return r.data;
		},
		refetchOnWindowFocus: false
	}));

	const filteredCountries = $derived.by(() => {
		const all = countriesQuery.data ?? [];
		const q = mirrorSearchInput.trim().toLowerCase();
		if (!q.length) return all;
		return all.filter((c) => c.toLowerCase().includes(q));
	});

	const mirrorMutation = createMutation(() => ({
		mutationFn: async () => {
			const list = countriesQuery.data ?? [];
			const country = sanitizeCountryChoice(list, mirrorCountryPick);
			if (!country) throw new Error('Pick a valid mirror country from the list.');
			const r = await refreshMirror(country);
			if (!r.success) throw new Error(r.error);
			return r.data;
		}
	}));

	const dnsMutation = createMutation(() => ({
		mutationFn: async () => {
			const r = await changeDns(dnsPick);
			if (!r.success) throw new Error(r.error);
		},
		onSuccess: () => {
			void dnsSummaryQuery.refetch();
		}
	}));

	const swapMutation = createMutation(() => ({
		mutationFn: async (enabled: boolean) => {
			const r = await toggleSwap(enabled);
			if (!r.success) throw new Error(r.error);
		},
		onSuccess: () => {
			void swapQuery.refetch();
		}
	}));

	let cacheConfirmOpen = $state(false);

	const cacheMutation = createMutation(() => ({
		mutationFn: async () => {
			const r = await cleanPackageCache();
			if (!r.success) throw new Error(r.error);
			return r.data;
		},
		onSettled: () => {
			cacheConfirmOpen = false;
		}
	}));

	const cpuMutation = createMutation(() => ({
		mutationFn: async () => {
			const r = await setCpuProfile(cpuPick);
			if (!r.success) throw new Error(r.error);
		},
		onSuccess: () => {
			void governorQuery.refetch();
		}
	}));

</script>

<main class="max-w-3xl mx-auto flex flex-col gap-6 pb-24">
	<div class="flex flex-row items-center gap-x-2 shrink-0">
		<Settings class="text-[#54CD4C] size-8" />
		<h1 class="text-2xl font-semibold">System settings</h1>
	</div>
	<p class="text-sm text-[#99A1AF] -mt-2">
		Mirrors, DNS, ZRAM swap, package cache, and CPU governor. Privileged actions use polkit
		(<code class="text-xs">pkexec</code>).
	</p>

	<!-- Mirrors -->
	<Card.Root class="bg-card border">
		<Card.Header>
			<Card.Title class="flex items-center gap-2 text-lg">
				<Shuffle class="size-5 text-[#54CD4C]" />
				Pacman mirrors
			</Card.Title>
			<Card.Description class="text-[#99A1AF]">
				Refresh the mirror list with Reflector (country allowlist). May take a while.
			</Card.Description>
		</Card.Header>
		<Card.Content class="flex flex-col gap-3">
			{#if countriesQuery.isLoading}
				<Skeleton class="h-10 w-full rounded-md" />
			{:else if countriesQuery.isError}
				<p class="text-destructive text-sm">Failed to load countries. Use retry.</p>
				<Button variant="outline" size="sm" onclick={() => countriesQuery.refetch()}>Retry</Button>
			{:else}
				<label class="text-xs text-muted-foreground" for="mirror-filter">Filter list</label>
				<input
					id="mirror-filter"
					class={cn(
						'w-full rounded-md border border-input bg-background px-3 py-2 text-sm',
						'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring'
					)}
					placeholder="Type to filter…"
					bind:value={mirrorSearchInput}
				/>
				<div class="flex flex-col sm:flex-row gap-2 sm:items-end">
					<div class="flex-1 min-w-0">
						<p class="text-xs text-muted-foreground mb-1">Country</p>
						<Select.Root type="single" bind:value={mirrorCountryPick}>
							<Select.Trigger class="w-full min-w-0">
								<span data-slot="select-value" class="truncate">{mirrorCountryPick || 'Select'}</span>
							</Select.Trigger>
							<Select.Content class="max-h-60 overflow-y-auto">
								{#each filteredCountries as c (c)}
									<Select.Item value={c} label={c}>{c}</Select.Item>
								{/each}
							</Select.Content>
						</Select.Root>
					</div>
					<Button
						class="shrink-0"
						disabled={mirrorMutation.isPending || !mirrorCountryPick}
						onclick={() => mirrorMutation.mutate()}
					>
						{#if mirrorMutation.isPending}
							<LoaderCircle class="size-4 animate-spin mr-2" />
							Refreshing…
						{:else}
							Apply mirror refresh
						{/if}
					</Button>
				</div>
			{/if}
			{#if mirrorMutation.isError && mirrorMutation.error}
				<p class="text-destructive text-sm">{mirrorMutation.error.message}</p>
			{/if}
			{#if mirrorMutation.isSuccess}
				<p class="text-green-600 text-sm">Mirror list refreshed.</p>
			{/if}
		</Card.Content>
	</Card.Root>

	<!-- DNS -->
	<Card.Root class="bg-card border">
		<Card.Header>
			<Card.Title class="flex items-center gap-2 text-lg">
				<Droplets class="size-5 text-[#54CD4C]" />
				DNS
			</Card.Title>
			<Card.Description class="text-[#99A1AF]">
				Switches resolver lines (elevated helper). Reloading networks may be needed.
			</Card.Description>
		</Card.Header>
		<Card.Content class="flex flex-col gap-3">
			<div
				class="flex gap-2 rounded-md border border-amber-500/40 bg-amber-500/10 px-3 py-2 text-sm text-foreground"
				role="status"
			>
				<AlertTriangle class="size-5 shrink-0 text-amber-600" aria-hidden />
				Writing <code>/etc/resolv.conf</code> can affect connectivity until you reconnect. Some sessions need a
				network restart.
			</div>
			<div class="flex flex-col sm:flex-row gap-3 sm:items-end">
				<div class="flex-1">
					<p class="text-xs text-muted-foreground mb-1">Provider</p>
					<Select.Root type="single" bind:value={dnsPick}>
						<Select.Trigger class="w-full">
							{dnsPick}
						</Select.Trigger>
						<Select.Content>
							{#each dnsOptions as p (p)}
								<Select.Item value={p} label={p}>{p}</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>
				</div>
				<div class="flex gap-2">
					<Button variant="outline" onclick={() => dnsSummaryQuery.refetch()} disabled={dnsSummaryQuery.isFetching}>
						{#if dnsSummaryQuery.isFetching}
							<LoaderCircle class="size-4 animate-spin" />
						{:else}
							Refresh status
						{/if}
					</Button>
					<Button disabled={dnsMutation.isPending} onclick={() => dnsMutation.mutate()}>
						{#if dnsMutation.isPending}
							<LoaderCircle class="size-4 animate-spin mr-2" />
						{:else}
							Apply
						{/if}
					</Button>
				</div>
			</div>
			<div class="text-sm text-muted-foreground space-x-2">
				<strong class="text-foreground">Current nameservers:</strong>
				{#if dnsSummaryQuery.isLoading}
					<span>…</span>
				{:else}
					<span>{dnsSummaryQuery.data ?? '—'}</span>
				{/if}
			</div>
			{#if dnsMutation.isError && dnsMutation.error}
				<p class="text-destructive text-sm">{dnsMutation.error.message}</p>
			{/if}
			{#if dnsMutation.isSuccess}
				<p class="text-green-600 text-sm">DNS update completed.</p>
			{/if}
		</Card.Content>
	</Card.Root>

	<!-- Swap -->
	<Card.Root class="bg-card border">
		<Card.Header>
			<Card.Title class="flex items-center gap-2 text-lg">
				<Zap class="size-5 text-[#54CD4C]" aria-hidden />
				ZRAM swap
			</Card.Title>
			<Card.Description class="text-[#99A1AF]">
				Toggles systemd zram-generator config via <code class="text-xs">modularitea-swap</code>.
			</Card.Description>
		</Card.Header>
		<Card.Content class="flex flex-col gap-3">
			{#if swapQuery.isLoading}
				<Skeleton class="h-8 w-40 rounded-md" />
			{:else}
				<label class="flex items-center gap-3 cursor-pointer select-none">
					<input
						type="checkbox"
						class="size-5 rounded border-input accent-[#54CD4C]"
						checked={Boolean(swapQuery.data)}
						disabled={swapMutation.isPending}
						onchange={(e) =>
							swapMutation.mutate((e.currentTarget as HTMLInputElement).checked)}
					/>
					<span class="text-sm">
						ZRAM swap enabled
						{#if swapMutation.isPending}
							<LoaderCircle class="inline-block size-4 animate-spin ml-2 align-middle" />
						{/if}
					</span>
				</label>
			{/if}
			{#if swapMutation.isError && swapMutation.error}
				<p class="text-destructive text-sm">{swapMutation.error.message}</p>
			{/if}
		</Card.Content>
	</Card.Root>

	<!-- Cache -->
	<Card.Root class="bg-card border">
		<Card.Header>
			<Card.Title class="flex items-center gap-2 text-lg">
				<Trash2 class="size-5 text-[#54CD4C]" />
				Package cache
			</Card.Title>
			<Card.Description class="text-[#99A1AF]">Clears <code>/var/cache/pacman/pkg</code>.</Card.Description>
		</Card.Header>
		<Card.Content class="flex flex-col gap-3">
			<Button variant="destructive" onclick={() => (cacheConfirmOpen = true)}>Clean cache…</Button>
			{#if cacheMutation.isError && cacheMutation.error}
				<p class="text-destructive text-sm">{cacheMutation.error.message}</p>
			{/if}
			{#if cacheMutation.isSuccess}
				<p class="text-green-600 text-sm">Cache cleaned.</p>
			{/if}
		</Card.Content>
	</Card.Root>

	<!-- CPU -->
	<Card.Root class="bg-card border">
		<Card.Header>
			<Card.Title class="flex items-center gap-2 text-lg">
				<Cpu class="size-5 text-[#54CD4C]" />
				CPU governor
			</Card.Title>
			<Card.Description class="text-[#99A1AF]">
				Uses <code class="text-xs">cpupower frequency-set</code> (via libs helper).
			</Card.Description>
		</Card.Header>
		<Card.Content class="flex flex-col gap-3">
			<p class="text-sm text-muted-foreground">
				Active governor: <strong class="text-foreground">{governorQuery.data ?? '…'}</strong>
				<Button variant="ghost" size="sm" class="ml-2 h-7 px-2" onclick={() => governorQuery.refetch()}>
					Refresh
				</Button>
			</p>
			<div class="flex flex-col sm:flex-row gap-3 sm:items-end">
				<div class="flex-1">
					<p class="text-xs text-muted-foreground mb-1">Profile</p>
					<Select.Root type="single" bind:value={cpuPick}>
						<Select.Trigger class="w-full">{cpuPick}</Select.Trigger>
						<Select.Content>
							{#each ['powersave', 'ondemand', 'performance'] as p (p)}
								<Select.Item value={p} label={p}>{p}</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>
				</div>
				<Button disabled={cpuMutation.isPending} onclick={() => cpuMutation.mutate()}>
					{#if cpuMutation.isPending}
						<LoaderCircle class="size-4 animate-spin mr-2" />
						Applying…
					{:else}
						Apply
					{/if}
				</Button>
			</div>
			{#if cpuMutation.isError && cpuMutation.error}
				<p class="text-destructive text-sm">{cpuMutation.error.message}</p>
			{/if}
		</Card.Content>
	</Card.Root>
</main>

<AlertDialog.Root bind:open={cacheConfirmOpen}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Clean package cache?</AlertDialog.Title>
			<AlertDialog.Description>This removes downloaded pacman packages. They will be re-fetched on next install.</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel onclick={() => (cacheConfirmOpen = false)}>Cancel</AlertDialog.Cancel>
			<AlertDialog.Action onclick={() => cacheMutation.mutate()}>
				{#if cacheMutation.isPending}
					<LoaderCircle class="size-4 animate-spin mr-2" />
				{/if}
				Clean
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
