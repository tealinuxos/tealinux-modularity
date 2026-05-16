// TODO: Save what the DNS is selected using store

<script lang="ts">
	import { ShieldCheck } from '@lucide/svelte';
	import * as Card from '$lib/components/ui/card';
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import type { DnsProvider } from '$lib/commands';

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

	let selected = $state<DnsProvider>('cloudflare');
</script>

<Card.Root class="h-[24.3rem]">
	<Card.Header class="pb-3">
		<div class="flex items-center gap-3">
			<div class="flex size-9 shrink-0 items-center justify-center rounded-lg bg-green-500/10">
				<ShieldCheck class="size-4 text-green-600" />
			</div>
			<div>
				<Card.Title class="text-base">DNS Configuration</Card.Title>
				<Card.Description class="text-xs">Preferred resolver</Card.Description>
			</div>
		</div>
	</Card.Header>

	<Card.Content class="overflow-scroll">
		<RadioGroup.Root bind:value={selected} class="space-y-2">
			{#each Object.entries(providers) as [key, provider] (key)}
				<label
					for="dns-{key}"
					class="flex cursor-pointer items-center justify-between rounded-lg border p-3 transition-all
						{selected === key ? 'border-primary bg-primary/5' : 'border-border bg-muted/20 hover:bg-muted/40'}"
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
