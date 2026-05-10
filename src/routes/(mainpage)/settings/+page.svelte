<script lang="ts">
	import { Settings2 } from '@lucide/svelte';
	import { Separator } from '$lib/components/ui/separator';

	import SystemTheme from './partials/system-theme.svelte';
	import PackageCache from './partials/package-cache.svelte';
	import SwapMemory from './partials/swap-memory.svelte';
	import MirrorSettings from './partials/mirror-settings.svelte';
	import DnsConfiguration from './partials/dns-configuration.svelte';
	import CpuPerformance from './partials/cpu-performance.svelte';
</script>

<div class="space-y-6 pb-6">
	<div class="flex items-center gap-3">
		<div class="flex size-10 shrink-0 items-center justify-center rounded-xl bg-primary/10">
			<Settings2 class="size-5 text-primary" />
		</div>
		<div>
			<h1 class="text-xl font-semibold tracking-tight">System Settings</h1>
			<p class="text-sm text-muted-foreground">Configure your TealinuxOS preferences</p>
		</div>
	</div>

	<Separator />

	<div class="grid grid-cols-1 gap-4 md:grid-cols-3">
		<div class="md:col-span-2">
			<PackageCache />
		</div>

		<SwapMemory />
		<SystemTheme />
		<MirrorSettings />
		<DnsConfiguration />

		<div class="md:col-span-3">
			<CpuPerformance />
		</div>
	</div>
</div>
