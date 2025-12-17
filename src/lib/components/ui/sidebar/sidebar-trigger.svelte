<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import { cn } from '$lib/utils.js';
	import PanelLeftIcon from '@lucide/svelte/icons/panel-left';
	import type { ComponentProps } from 'svelte';
	import { useSidebar } from './context.svelte.js';
	import * as ButtonGroup from '$lib/components/ui/button-group/index';
	import { Minus, Square, X } from '@lucide/svelte';
	import { getCurrentWindow } from '@tauri-apps/api/window';

	let {
		ref = $bindable(null),
		class: className,
		onclick,
		...restProps
	}: ComponentProps<typeof Button> & {
		onclick?: (e: MouseEvent) => void;
	} = $props();

	const sidebar = useSidebar();
	const appWindow = getCurrentWindow();
</script>

<nav class="border-b h-[57px] bg-sidebar text-sidebar-foreground flex flex-row justify-end">
	<!-- TODO: Search Component -->
	<!-- <p>test</p> -->
	<ButtonGroup.Root class="h-full" aria-label="Button group">
		<Button onclick={async () => await appWindow.minimize()} class="h-full" variant="ghost">
			<Minus />
		</Button>
		<Button onclick={async () => await appWindow.maximize()} class="h-full" variant="ghost">
			<Square />
		</Button>
		<Button
			onclick={async () => await appWindow.close()}
			class="h-full hover:bg-red-400 hover:text-red-900"
			variant="ghost"
		>
			<X />
		</Button>
	</ButtonGroup.Root>

</nav>
<!-- px-4 py-4  -->
<!-- <Button
	data-sidebar="trigger"
	data-slot="sidebar-trigger"
	variant="ghost"
	size="icon"
	class={cn("size-7", className)}
	type="button"
	onclick={(e) => {
		onclick?.(e);
		sidebar.toggle();
	}}
	{...restProps}
> -->
<!-- <PanelLeftIcon /> -->
<!-- <span class="sr-only">Toggle Sidebar</span> -->
<!-- </Button> -->
