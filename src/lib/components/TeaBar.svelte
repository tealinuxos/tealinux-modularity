<script lang="ts">
	import * as Sidebar from '$lib/components/ui/sidebar/index';
</script>

<Sidebar.Root>
	<Sidebar.Header class="flex flex-row gap-x-1 items-center px-4 py-3 border-b">
		<img class="size-8" src="tealinux.svg" alt="TeaLinuxOS Logo" />
		<p class="text-[#54CD4C] font-bold text-xl">TeaLinuxOS</p>
	</Sidebar.Header>
	<Sidebar.Content>
		<Sidebar.Group />
		<Sidebar.Group />
	</Sidebar.Content>
	<Sidebar.Footer>
		<p>Settings</p>
	</Sidebar.Footer>
</Sidebar.Root>
