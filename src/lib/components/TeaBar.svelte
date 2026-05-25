<script lang="ts">
	import * as Sidebar from '$lib/components/ui/sidebar/index';
	import { cn } from '$lib/utils';
	import { teabarItems } from './teabar-items';
	import { Button } from './ui/button';
	import { page } from '$app/state';
	import { Cog } from '@lucide/svelte';
	import { useSidebar } from './ui/sidebar/context.svelte';

	const sidebar = useSidebar();

	const isActive = (targetPath: string) => {
		const current = page.url.pathname;
		return current.startsWith(targetPath);
	};

	let previousWidth = $state(typeof window !== 'undefined' ? window.innerWidth : 1280);

	$effect(() => {
		if (typeof window === 'undefined') return;

		const handleResize = () => {
			const currentWidth = window.innerWidth;
			const collapseBreakpoint = 1024; 

			if (currentWidth <= collapseBreakpoint && previousWidth > collapseBreakpoint) {
				sidebar.setOpen(false);
			}
			else if (currentWidth > collapseBreakpoint && previousWidth <= collapseBreakpoint) {
				sidebar.setOpen(true);
			}
			previousWidth = currentWidth;
		};

		window.addEventListener('resize', handleResize);

		if (window.innerWidth <= 1024 && sidebar.state === 'expanded') {
			sidebar.setOpen(false);
		}

		return () => window.removeEventListener('resize', handleResize);
	});
</script>

<Sidebar.Root>
	<Sidebar.Header class="flex flex-row gap-x-1 items-center px-4 py-3 border-b">
		<img class="size-9 rounded-[4px]" src="modularitea.png" alt="Modularitea Logo" />
		<p class="text-[#26A768] font-bold text-xl">Modularitea</p>
	</Sidebar.Header>
	<Sidebar.Content class="px-3.5 py-3">
		{#each teabarItems as item (item.href)}
			<Button
				href={item.href}
				variant="ghost"
				class={cn(
					'flex flex-row justify-start w-full py-5 items-center gap-x-3 transition-all duration-300',
					sidebar.state === 'collapsed' && 'justify-center px-0',
					isActive(item.href) && 'bg-accent text-accent-foreground dark:bg-accent/50'
				)}
			>
				<item.icon class="size-5 shrink-0" />
				{#if sidebar.state === 'expanded'}
					<p class="truncate">{item.label}</p>
				{/if}
			</Button>
		{/each}
	</Sidebar.Content>
	<Sidebar.Footer class="border-t">
		<Button
			href="/settings"
			variant="ghost"
			class={cn(
				'flex flex-row justify-start w-full py-5 items-center gap-x-3 transition-all duration-300',
				sidebar.state === 'collapsed' && 'justify-center px-0',
				isActive('/settings') && 'bg-accent text-accent-foreground dark:bg-accent/50'
			)}
		>
			<Cog class="size-5 shrink-0" />
			{#if sidebar.state === 'expanded'}
				<p class="truncate">Settings</p>
			{/if}
		</Button>
	</Sidebar.Footer>
</Sidebar.Root>
