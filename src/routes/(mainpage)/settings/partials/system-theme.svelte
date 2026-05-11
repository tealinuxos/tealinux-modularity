<script lang="ts">
	import { Palette, Sun, Monitor, Moon } from '@lucide/svelte';
	import * as Card from '$lib/components/ui/card';
	import { setMode, mode } from 'mode-watcher';

	type Theme = 'light' | 'system' | 'dark';

	let current = $state<Theme>((mode.current as Theme) ?? 'system');

	$effect(() => {
		if (mode.current) current = mode.current as Theme;
	});

	function select(t: Theme) {
		current = t;
		setMode(t);
	}

	const themes: { value: Theme; label: string; Icon: typeof Sun }[] = [
		{ value: 'light', label: 'Light', Icon: Sun },
		{ value: 'system', label: 'Auto', Icon: Monitor },
		{ value: 'dark', label: 'Dark', Icon: Moon }
	];
</script>

<Card.Root class="h-full">
	<Card.Header class="pb-3">
		<div class="flex items-center gap-3">
			<div class="flex size-9 shrink-0 items-center justify-center rounded-lg bg-primary/10">
				<Palette class="size-4 text-primary" />
			</div>
			<div>
				<Card.Title class="text-base">System Theme</Card.Title>
				<Card.Description class="text-xs">Appearance preference</Card.Description>
			</div>
		</div>
	</Card.Header>

	<Card.Content class="space-y-4">
		<p class="text-xs leading-relaxed text-muted-foreground">
			Auto mode switches between Light and Dark based on your system schedule.
		</p>
		<div class="grid grid-cols-3 gap-2">
			{#each themes as { value, label, Icon } (value)}
				<button
					onclick={() => select(value)}
					class="flex flex-col items-center gap-2 rounded-lg border px-2 py-3 text-xs font-medium transition-all
						{current === value
						? 'border-primary bg-primary/10 text-primary'
						: 'border-border bg-muted/30 text-muted-foreground hover:bg-muted/60 hover:text-foreground'}"
				>
					<Icon class="size-4" />
					{label}
				</button>
			{/each}
		</div>
	</Card.Content>
</Card.Root>
