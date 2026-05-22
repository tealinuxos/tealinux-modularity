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

<nav
	data-tauri-drag-region
	class="border-b h-[57px] bg-sidebar text-sidebar-foreground flex flex-row justify-between items-center sticky top-0 z-999 w-full px-2"
>
	<div class="flex items-center gap-2">
		<Button
			variant="ghost"
			size="icon"
			class={cn(
				'h-9 w-9 transition-transform duration-300',
				sidebar.state === 'collapsed' && 'rotate-180'
			)}
			onclick={() => sidebar.toggle()}
		>
			<PanelLeftIcon class="size-5" />
			<span class="sr-only">Toggle Sidebar</span>
		</Button>
	</div>

	<ButtonGroup.Root class="h-full" aria-label="Button group">
		<Button onclick={async () => await appWindow.minimize()} class="h-full px-4" variant="ghost">
			<Minus class="size-4" />
		</Button>
		<Button
			onclick={async () => await appWindow.toggleMaximize()}
			class="h-full px-4"
			variant="ghost"
		>
			<Square class="size-4" />
		</Button>
		<Button
			onclick={async () => await appWindow.close()}
			class="h-full px-4 hover:bg-red-500 dark:hover:bg-red-600 hover:text-white transition-colors"
			variant="ghost"
		>
			<X class="size-4" />
		</Button>
	</ButtonGroup.Root>
</nav>
