<script lang="ts">
	import { ShieldCheck, LoaderCircle } from '@lucide/svelte';
	import * as Card from '$lib/components/ui/card';
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import { commands, type DnsProvider } from '$lib/commands';
	import { settingsState } from '$lib/state/settings.svelte';
	import { toast } from 'svelte-sonner';
	import { onMount } from 'svelte';

	$inspect(settingsState.dnsProvider);

	type DNS_VALUE = {
		name: string;
		desc: string;
		address: string;
	};

	const providers: Record<DnsProvider, DNS_VALUE> = {
		cloudflare: { name: 'Cloudflare', address: '1.1.1.1', desc: 'Privacy-focused' },
		google: { name: 'Google DNS', address: '8.8.8.8', desc: 'High availability' },
		quad9: { name: 'Quad9', address: '9.9.9.9', desc: 'Security filtering' },
		opendns: { name: 'OpenDNS', address: '208.67.222.222', desc: 'Content filtering' },
		adguard: { name: 'AdGuard DNS', address: '94.140.14.14', desc: 'Ad & tracker blocking' }
	};

	let updating = $state(false);
	let loaded = $state(false);

	onMount(async () => {
		try {
			const result = await commands.getCurrentDnsProvider();
			if (result.status === 'ok') {
				settingsState.dnsProvider = result.data;
			}
		} catch (err) {
			console.error('Failed to fetch DNS provider:', err);
		} finally {
			loaded = true;
		}
	});

	async function handleDnsChange(value: string) {
		const provider = value as DnsProvider;
		if (settingsState.dnsProvider === provider) return;

		updating = true;
		try {
			const result = await commands.switchDns(provider);
			if (result.status === 'ok') {
				settingsState.dnsProvider = provider;
				toast.success('DNS Updated', {
					description: `Now using ${providers[provider].name} as your DNS resolver.`
				});
			} else {
				toast.error('Failed to update DNS settings.', {
					description: 'Administrator privileges are required.'
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

<Card.Root class="h-[24.3rem]">
	<Card.Header class="pb-3">
		<div class="flex items-center gap-3">
			<div class="flex size-9 shrink-0 items-center justify-center rounded-lg bg-green-500/10">
				<ShieldCheck class="size-4 text-green-600" />
			</div>
			<div>
				<div class="flex items-center gap-2">
					<Card.Title class="text-base">DNS Configuration</Card.Title>
					{#if updating}
						<LoaderCircle class="size-3 animate-spin text-muted-foreground" />
					{/if}
				</div>
				<Card.Description class="text-xs">
					{#if !loaded}
						Detecting…
					{:else if settingsState.dnsProvider === null}
						Custom DNS Detected
					{:else}
						Preferred resolver
					{/if}
				</Card.Description>
			</div>
		</div>
	</Card.Header>

	<Card.Content class="overflow-scroll">
		<RadioGroup.Root
			value={settingsState.dnsProvider ?? undefined}
			onValueChange={handleDnsChange}
			disabled={updating || !loaded}
			class="space-y-2"
		>
			{#each Object.entries(providers) as [key, provider] (key)}
				{@const isSelected = settingsState.dnsProvider === key}
				{@const isLoading = !loaded}
				<label
					for="dns-{key}"
					class="flex cursor-pointer items-center justify-between rounded-lg border p-3 transition-all
						{isSelected ? 'border-primary bg-primary/5' : 'border-border bg-muted/20 hover:bg-muted/40'}
						{updating || isLoading ? 'opacity-50 cursor-not-allowed' : ''}"
				>
					<div class="flex items-center gap-3">
						<RadioGroup.Item value={key} id="dns-{key}" />
						<div>
							<p class="text-sm font-medium leading-none">{provider.name}</p>
							<p class="mt-0.5 text-xs text-muted-foreground">{provider.desc}</p>
						</div>
					</div>
					<code class="rounded bg-muted px-1.5 py-0.5 font-mono text-[10px] text-muted-foreground">
						{provider.address}
					</code>
				</label>
			{/each}
		</RadioGroup.Root>
	</Card.Content>
</Card.Root>
