<script lang="ts">
	import { SlidersHorizontal, LoaderCircle } from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import * as Carousel from '$lib/components/ui/carousel';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { onMount } from 'svelte';
	import { cn } from '$lib/utils';
	import { fetchThemes, applyTheme } from '$lib/services/grub.service';
	import { briefErrorMessage } from '$lib/utils/frontend-error-msg';
	import type { ThemeManifest } from '$lib/commands';

	const listThemes = $state({
		data: [] as ThemeManifest[],
		isLoading: true,
		error: null as string | null
	});

	const selectedTheme = $state({
		theme: null as ThemeManifest | null
	});

	const installState = $state({
		isLoading: false,
		error: null as string | null,
		success: false
	});

	let dialogOpen = $state(false);

	const selectedPreview = $derived.by(() => {
		return (
			selectedTheme.theme?.preview_image ??
			'https://placehold.co/1280x720/transparent/FFFFFF?text=Select+Theme+First!&font=montserrat'
		);
	});

	const selectedThemeName = $derived(selectedTheme.theme?.name);

	type DialogPhase = 'confirm' | 'loading' | 'error' | 'success';

	const dialogPhase = $derived.by<DialogPhase>(() => {
		if (installState.isLoading) return 'loading';
		if (installState.success) return 'success';
		if (installState.error !== null) return 'error';
		return 'confirm';
	});

	const resetInstallState = () => {
		installState.isLoading = false;
		installState.error = null;
		installState.success = false;
	};

	const openDialog = () => {
		resetInstallState();
		dialogOpen = true;
	};

	const closeDialog = () => {
		if (installState.isLoading) return;
		resetInstallState();
		dialogOpen = false;
	};

	const themeClickHandler = (theme: ThemeManifest) => {
		selectedTheme.theme = theme;
	};

	const installThemeHandler = async () => {
		if (!selectedTheme.theme?.name) return;

		installState.isLoading = true;
		installState.error = null;
		installState.success = false;

		const result = await applyTheme(selectedTheme.theme.name);

		installState.isLoading = false;

		if (result.success) {
			installState.success = true;
			setTimeout(() => {
				dialogOpen = false;
				resetInstallState();
			}, 2000);
		} else {
			installState.error = briefErrorMessage(result.code, result.error);
		}
	};

	let previewBroken = $state(false);
	let themeFilter = $state('');

	const visibleThemes = $derived.by(() => {
		const q = themeFilter.trim().toLowerCase();
		if (!q) return listThemes.data;
		return listThemes.data.filter((t) => t.name.toLowerCase().includes(q));
	});

	let filterDebounce: ReturnType<typeof setTimeout> | undefined;
	let themeFilterInput = $state('');
	$effect(() => {
		const v = themeFilterInput;
		clearTimeout(filterDebounce);
		filterDebounce = setTimeout(() => {
			themeFilter = v;
		}, 200);
		return () => clearTimeout(filterDebounce);
	});

	onMount(async () => {
		const result = await fetchThemes();
		if (result.success) {
			listThemes.data = result.data;
			selectedTheme.theme = result.data[0];
		} else {
			listThemes.error = briefErrorMessage(result.code, result.error);
		}
		listThemes.isLoading = false;
	});
</script>

<main>
	<div class="flex flex-row items-center gap-x-2 mb-4 shrink-0">
		<SlidersHorizontal class="text-[#54CD4C] size-8" />
		<h1 class="text-2xl font-semibold">GRUB Theme Changer</h1>
	</div>

	<Card.Root class="h-[calc(100vh-137px)] w-full bg-card flex flex-col py-0">
		<Card.Content class="flex-1 min-h-0 flex flex-col p-4 gap-y-4">
			<section class="flex flex-col flex-1 min-h-0">
				<p class="text-[#99A1AF] mb-2 shrink-0">Preview Selected Theme</p>
				<div
					class="relative w-full h-full rounded-md border flex items-center justify-center overflow-hidden bg-muted/30"
				>
					<img
						src={previewBroken ? 'https://placehold.co/1280x720/cccccc/333333?text=Preview+unavailable' : selectedPreview}
						alt="Preview"
						loading="lazy"
						referrerpolicy="no-referrer"
						class="absolute inset-0 h-full w-full object-contain"
						onerror={() => {
							previewBroken = true;
						}}
						onload={() => {
							previewBroken = false;
						}}
					/>
				</div>
			</section>

			<section class="flex flex-col shrink-0 gap-2">
				<p class="text-[#99A1AF]">Choose A Theme</p>
				<input
					class={cn(
						'w-full max-w-md rounded-md border border-input bg-background px-3 py-2 text-sm',
						'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring'
					)}
					placeholder="Search themes…"
					bind:value={themeFilterInput}
				/>
				{#if listThemes.error}
					<div class="rounded-md border border-destructive/40 bg-destructive/10 p-4 flex flex-col gap-2 items-start">
						<p class="text-sm text-destructive">{listThemes.error}</p>
						<Button
							size="sm"
							variant="outline"
							onclick={async () => {
								listThemes.isLoading = true;
								listThemes.error = null;
								const result = await fetchThemes();
								if (result.success) {
									listThemes.data = result.data;
									selectedTheme.theme = result.data[0];
								} else {
									listThemes.error = briefErrorMessage(result.code, result.error);
								}
								listThemes.isLoading = false;
							}}
						>
							Try Again
						</Button>
					</div>
				{/if}
				<Carousel.Root opts={{ align: 'start', skipSnaps: true }} class="w-full">
					<Carousel.Content>
						{#if listThemes.isLoading}
							{#each { length: 5 } as _, i (i)}
								{@render SkeletonCard()}
							{/each}
						{:else if visibleThemes.length === 0}
							<p class="text-sm text-muted-foreground py-6 px-2">No themes match your search.</p>
						{:else}
							{#each visibleThemes as theme (theme.name)}
								<Carousel.Item
									onclick={() => themeClickHandler(theme)}
									class="basis-[85%] sm:basis-1/2 md:basis-1/3 lg:basis-1/4"
								>
									<div class="p-1">
										<Card.Root
											class={cn(
												'relative flex aspect-video items-center justify-center p-0 overflow-hidden rounded-xl border-4 border-transparent hover:border-primary cursor-pointer transition-all',
												selectedTheme.theme?.name === theme.name ? 'border-primary' : ''
											)}
										>
											<img
												src={theme.preview_image ?? ''}
												loading="lazy"
												referrerpolicy="no-referrer"
												alt={theme.name}
												class="absolute inset-0 h-full w-full object-cover bg-muted"
												onerror={(e) => {
													(e.currentTarget as HTMLImageElement).src =
														'https://placehold.co/640x360/e2e8f0/64748b?text=Preview';
												}}
											/>
										</Card.Root>
									</div>
								</Carousel.Item>
							{/each}
						{/if}
					</Carousel.Content>
				</Carousel.Root>
			</section>
		</Card.Content>

		<Card.Footer class="pb-4 px-4 pt-0 shrink-0">
			{@render ApplyButton()}
		</Card.Footer>
	</Card.Root>
</main>

{#snippet SkeletonCard()}
	<Carousel.Item class="basis-[85%] sm:basis-1/2 md:basis-1/3 lg:basis-1/4">
		<div class="p-1">
			<Skeleton class="aspect-video w-full rounded-xl" />
		</div>
	</Carousel.Item>
{/snippet}

{#snippet ApplyButton()}
	<Button
		class="w-full"
		disabled={listThemes.isLoading || !selectedTheme.theme}
		onclick={openDialog}
	>
		Apply Now
	</Button>

	<AlertDialog.Root
		open={dialogOpen}
		onOpenChange={(open) => {
			if (!open) closeDialog();
		}}
	>
		<AlertDialog.Content>
			<AlertDialog.Header>
				<AlertDialog.Title>
					{#if dialogPhase === 'loading'}
						Installing Theme...
					{:else if dialogPhase === 'success'}
						✓ Theme Applied!
					{:else if dialogPhase === 'error'}
						Failed to Apply Theme
					{:else}
						Are you absolutely sure?
					{/if}
				</AlertDialog.Title>

				<AlertDialog.Description>
					{#if dialogPhase === 'loading'}
						Applying <strong>{selectedThemeName}</strong>, please wait and do not close this window.
					{:else if dialogPhase === 'success'}
						<span class="text-green-600 dark:text-green-400">
							The GRUB theme <strong>{selectedThemeName}</strong> has been successfully applied. This dialog
							will close automatically.
						</span>
					{:else if dialogPhase === 'error'}
						<span class="text-destructive">
							{installState.error}
						</span>
					{:else}
						This action will apply the selected theme to your GRUB configuration (polkit / pkexec). Current theme:
						<span class="underline underline-offset-2">{selectedThemeName}</span>
					{/if}
				</AlertDialog.Description>
			</AlertDialog.Header>

			<AlertDialog.Footer>
				{#if dialogPhase === 'confirm'}
					<AlertDialog.Cancel onclick={closeDialog}>Cancel</AlertDialog.Cancel>
					<AlertDialog.Action onclick={installThemeHandler}>Continue</AlertDialog.Action>
				{:else if dialogPhase === 'loading'}
					<AlertDialog.Cancel disabled={true}>Cancel</AlertDialog.Cancel>
					<AlertDialog.Action disabled={true}>
						<LoaderCircle class="mr-2 size-4 animate-spin" />
						Installing...
					</AlertDialog.Action>
				{:else if dialogPhase === 'error'}
					<AlertDialog.Cancel onclick={closeDialog}>Cancel</AlertDialog.Cancel>
					<Button onclick={installThemeHandler}>Try Again</Button>
				{:else if dialogPhase === 'success'}
					<AlertDialog.Cancel onclick={closeDialog}>Close</AlertDialog.Cancel>
				{/if}
			</AlertDialog.Footer>
		</AlertDialog.Content>
	</AlertDialog.Root>
{/snippet}
