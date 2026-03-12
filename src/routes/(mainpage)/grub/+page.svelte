<script lang="ts">
	import { SlidersHorizontal, LoaderCircle } from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card/index';
	import * as Carousel from '$lib/components/ui/carousel/index';
	import * as AlertDialog from '$lib/components/ui/alert-dialog/index';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { onMount } from 'svelte';
	import { cn } from '$lib/utils';
	import { commands, type LocalThemeManifest } from '$lib/commands';
	import { errorMessageMapper } from '$lib/utils/error-message-mapper';

	const listThemes = $state({
		data: [] as LocalThemeManifest[],
		isLoading: true,
		error: null as string | null
	});

	const selectedTheme = $state({
		index: 0 as number,
		theme: null as LocalThemeManifest | null
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

	const themeClickHandler = (theme: LocalThemeManifest, index: number) => {
		selectedTheme.index = index;
		selectedTheme.theme = theme;
	};

	const installThemeHandler = async () => {
		if (!selectedTheme.theme) return;

		installState.isLoading = true;
		installState.error = null;
		installState.success = false;

		try {
			const result = await commands.setGrubTheme(selectedThemeName!);

			if (result.status === 'ok') {
				installState.success = true;

				setTimeout(() => {
					dialogOpen = false;
					resetInstallState();
				}, 2000);
			} else if (result.status === 'error') {
				installState.error = errorMessageMapper(result.error);
			}
		} catch (e) {
			installState.error = e instanceof Error ? e.message : 'Failed to install theme';
		} finally {
			installState.isLoading = false;
		}
	};

	onMount(async () => {
		try {
			listThemes.data = await commands.getGrubThemes();
			selectedTheme.theme = listThemes.data[0];
		} catch (e) {
			listThemes.error = e instanceof Error ? e.message : 'Gagal memuat tema';
		} finally {
			listThemes.isLoading = false;
		}
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
						src={selectedPreview}
						alt="Preview"
						class="absolute inset-0 h-full w-full object-contain"
					/>
				</div>
			</section>

			<section class="flex flex-col shrink-0">
				<p class="text-[#99A1AF] mb-2">Choose A Theme</p>
				<Carousel.Root opts={{ align: 'start', skipSnaps: true }} class="w-full">
					<Carousel.Content>
						{#if listThemes.isLoading}
							{#each Array(5)}
								{@render SkeletonCard()}
							{/each}
						{:else}
							{#each listThemes.data as theme, i (theme.name)}
								<Carousel.Item
									onclick={() => themeClickHandler(theme, i)}
									class="basis-[85%] sm:basis-1/2 md:basis-1/3 lg:basis-1/4"
								>
									<div class="p-1">
										<Card.Root
											class={cn(
												'relative flex aspect-video items-center justify-center p-0 overflow-hidden rounded-xl border-4 border-transparent hover:border-primary cursor-pointer transition-all',
												selectedTheme.index === i ? 'border-primary' : ''
											)}
										>
											<img
												src={theme.preview_image}
												loading="lazy"
												alt={theme.name}
												class="absolute inset-0 h-full w-full object-cover"
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
						<span class="text-green-600">
							The GRUB theme <strong>{selectedThemeName}</strong> has been successfully applied! This
							dialog will close automatically.
						</span>
					{:else if dialogPhase === 'error'}
						<span class="text-destructive">
							{installState.error}
						</span>
					{:else}
						This action will apply the selected theme to your GRUB configuration. Current selected
						theme: <span class="underline underline-offset-2">{selectedThemeName}</span>
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
