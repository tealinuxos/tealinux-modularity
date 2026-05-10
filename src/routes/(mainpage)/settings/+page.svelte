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

<div class="space-y-6 pb-6 h-fit">
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

	<div class="grid grid-cols-3 grid-rows-2 gap-4">
		<div class="col-span-2">
			<PackageCache />
		</div>

		<div class="col-start-3">
			<SwapMemory />
		</div>

		<div class="row-start-2">
			<SystemTheme />
		</div>

		<div class="row-start-2">
			<MirrorSettings />
		</div>

		<div class="row-span-2 row-start-2">
			<DnsConfiguration />
		</div>

		<div class="col-span-2">
			<CpuPerformance />
		</div>
	</div>
</div>
