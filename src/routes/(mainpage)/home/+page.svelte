<script lang="ts">
	import { commands } from '$lib/commands';
	import { onMount } from 'svelte';

	onMount(async () => {
		const a = await commands.initConfigurationFile();

		console.log(a);
	});
</script>
