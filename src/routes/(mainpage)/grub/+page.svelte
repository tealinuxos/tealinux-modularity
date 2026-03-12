<script lang="ts">
	import { SlidersHorizontal } from '@lucide/svelte';
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

	const selectedPreview = $derived.by(() => {
		return (
			selectedTheme.theme?.preview_image ??
			'https://placehold.co/1280x720/transparent/FFFFFF?text=Select+Theme+First!&font=montserrat'
		);
	});

	const selectedThemeName = $derived(selectedTheme.theme?.name);

	const themeClickHandler = (theme: LocalThemeManifest, index: number) => {
		selectedTheme.index = index;
		selectedTheme.theme = theme;
	};

	const installThemeHandler = async (themeName: string) => {
		installState.isLoading = true;
		installState.error = null;
		installState.success = false;

		try {
			const result = await commands.setGrubTheme(themeName);

			if (result.status === 'ok') {
				installState.success = true;
				installState.error = null;
				console.log('Theme installed successfully:', result.data);

				setTimeout(() => {
					installState.success = false;
				}, 2000);
			} else if (result.status === 'error') {
				const errorMessage = errorMessageMapper(result.error);
				installState.error = errorMessage;
				console.error('Failed to install theme:', result.error);
			}
		} catch (e) {
			const errorMsg = e instanceof Error ? e.message : 'Failed to install theme';
			installState.error = errorMsg;
			console.error('Exception during theme installation:', e);
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
	<AlertDialog.Root open={!installState.success}>
		<AlertDialog.Trigger class="w-full">
			<Button class="w-full" disabled={listThemes.isLoading || !selectedTheme.theme}>
				{installState.isLoading ? 'Installing...' : 'Apply Now'}
			</Button>
		</AlertDialog.Trigger>
		<AlertDialog.Content>
			<AlertDialog.Header>
				<AlertDialog.Title>
					{installState.success ? '✓ Theme Applied Successfully!' : 'Are you absolutely sure?'}
				</AlertDialog.Title>
				<AlertDialog.Description>
					{#if installState.success}
						<span class="text-green-600"> The GRUB theme has been successfully applied! </span>
					{:else if installState.error}
						<span class="text-red-600">
							{installState.error}
						</span>
					{:else}
						This action will apply the selected theme to your GRUB configuration. Current selected
						theme: <span class="underline underline-offset-2">
							{selectedThemeName}
						</span>
					{/if}
				</AlertDialog.Description>
			</AlertDialog.Header>
			<AlertDialog.Footer>
				{#if !installState.success}
					<AlertDialog.Cancel disabled={installState.isLoading}>Cancel</AlertDialog.Cancel>
					<AlertDialog.Action
						disabled={installState.isLoading}
						onclick={() => selectedTheme.theme && installThemeHandler(selectedTheme.theme.name)}
					>
						{installState.isLoading ? 'Installing...' : 'Continue'}
					</AlertDialog.Action>
				{:else}
					<AlertDialog.Cancel>Close</AlertDialog.Cancel>
				{/if}
			</AlertDialog.Footer>
		</AlertDialog.Content>
	</AlertDialog.Root>
{/snippet}
