<script lang="ts">
	import * as Sidebar from '$lib/components/ui/sidebar/index';
<<<<<<< HEAD
	import { cn } from '$lib/utils';
	import { teabarItems } from './teabar-items';
	import { Button } from './ui/button';
	import { page } from '$app/state';
	import { Cog } from '@lucide/svelte';
=======
>>>>>>> 0ff51a826d7fb8de34b5f0af3d1d1a8aaae1d7de
</script>

<Sidebar.Root>
	<Sidebar.Header class="flex flex-row gap-x-1 items-center px-4 py-3 border-b">
		<img class="size-8" src="tealinux.svg" alt="TeaLinuxOS Logo" />
		<p class="text-[#54CD4C] font-bold text-xl">TeaLinuxOS</p>
	</Sidebar.Header>
<<<<<<< HEAD
	<Sidebar.Content class="px-3.5 py-3">
		{#each teabarItems as item (item.href)}
			<Button
				href={item.href}
				variant="ghost"
				class={cn(
					'flex flex-row justify-start w-full py-5 items-center',
					page.url.pathname === item.href && 'bg-accent text-accent-foreground dark:bg-accent/50'
				)}
			>
				<item.icon />
				<p>{item.label}</p>
			</Button>
		{/each}
	</Sidebar.Content>
	<Sidebar.Footer class="border-t">
		<Button
			href="/settings"
			variant="ghost"
			class={cn(
				'flex flex-row justify-start w-full py-5 items-center',
				page.url.pathname === '/settings' && 'bg-accent text-accent-foreground dark:bg-accent/50'
			)}
		>
			<Cog />
			<p>Settings</p>
		</Button>
=======
	<Sidebar.Content>
		<Sidebar.Group />
		<Sidebar.Group />
	</Sidebar.Content>
	<Sidebar.Footer>
		<p>Settings</p>
>>>>>>> 0ff51a826d7fb8de34b5f0af3d1d1a8aaae1d7de
	</Sidebar.Footer>
</Sidebar.Root>
