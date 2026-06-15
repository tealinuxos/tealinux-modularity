<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import TeaSplashBar from './partials/TeaSplashBar.svelte';
	import TeaSplashFooter from './partials/TeaSplashFooter.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Item from '$lib/components/ui/item/';
	import * as Dialog from '$lib/components/ui/dialog/';
	import {
		ChevronRight,
		Cog,
		Download,
		PanelsLeftBottom,
		Sparkle,
		Check,
		type Icon
	} from '@lucide/svelte';
	import { commands } from '$lib/commands';

	interface MenuProps {
		title: string;
		description: string;
		icon: typeof Icon;
	}

	const totalSteps = 3;

	const MENU_LISTS: MenuProps[] = [
		{
			title: 'Desktop Customization',
			description: 'Add or install other Desktop Environments for your maximum setup',
			icon: PanelsLeftBottom
		},
		{
			title: 'GRUB Theme Changer',
			description: 'Customize your linux bootloader theme with ease',
			icon: Cog
		},
		{
			title: 'Install Profile',
			description: 'No more specialized linux, feel free to change your linux profile',
			icon: Download
		},
		{
			title: 'Discover More',
			description: 'Dive into a world of features waiting for you to explore!',
			icon: Sparkle
		}
	];

	const CHANGELOG_LISTS: string[] = [
		'Interactive onboarding experience with multi-step introduction',
		'Modernized "Latest News" section with GitHub-inspired aesthetics',
		'Revamped GRUB Customizer featuring intuitive wallpaper selection',
		'Enhanced UI responsiveness with optimized fluid animations',
		'Improved visual contrast and readability for AMOLED themes'
	];

	let currentStep = $state(0);

	const nextHandler = async () => {
		if (currentStep < totalSteps - 1) {
			currentStep += 1;
			return;
		}

		const data = await commands.initConfigurationFile();

		if (data.status === 'ok') {
			await commands.showMainWindow();
		} else {
			//  TODO: MAKE IT TOAST / MODAL
			console.error('Error initializing configuration file:', data.error);
		}
	};

	const backHandler = () => {
		currentStep -= 1;
	};
</script>

<main class="min-h-screen w-full bg-[#0D0D0D] flex flex-col overflow-hidden text-white">
	<div class="shrink-0 px-[clamp(1rem,3vw,3rem)]">
		<TeaSplashBar />
	</div>

	<section class="flex-1 flex flex-col items-center justify-center w-full relative overflow-hidden">
		<div class="grid place-items-center w-full max-w-[90rem] mx-auto px-[clamp(1.5rem,4vw,5rem)]">
			{#if currentStep === 0}
				{@render StepsOne()}
			{:else if currentStep === 1}
				{@render StepsTwo()}
			{:else if currentStep === 2}
				{@render StepsThree()}
			{/if}
		</div>
	</section>

	<div class="shrink-0 px-[clamp(1rem,3vw,3rem)] pb-[clamp(0.75rem,2vh,2rem)]">
		<TeaSplashFooter {currentStep} {totalSteps} onBack={backHandler} onNext={nextHandler} />
	</div>
</main>

{#snippet StepsOne()}
	<div
		in:fly={{ x: 20, duration: 500, delay: 200 }}
		out:fade={{ duration: 200 }}
		class="col-start-1 row-start-1 flex items-center justify-center flex-col text-center w-full py-[clamp(1rem,4vh,4rem)]"
	>
		<h1
			class="shimmer text-[#999999] shimmer-color-[#FFFFFF] font-bold leading-none antialiased transform-gpu translate-z-0 backface-hidden"
			style="font-size: clamp(5rem, 20vw, 18rem);"
		>
			LILYA
		</h1>

		<div
			class="h-3 w-[20rem] rounded-full bg-linear-to-r from-[#26A768] via-[#3F9A39] to-[#2A6726] mb-8"
		></div>

		<p
			class="font-medium text-[#999999] tracking-[0.2em]"
			style="font-size: clamp(0.9rem, 1.6vw, 2rem);"
		>
			BY <span
				class="text-[#26A768] relative pr-6
      after:content-['']
      after:absolute
      after:w-4
      after:h-8
      after:bg-[url('/tealinux.svg')]
      after:bg-contain
      after:bg-no-repeat
      after:-top-1"
			>
				TealinuxOS
			</span>
		</p>

		<p
			class="font-medium tracking-[0.1em] text-[#999999] leading-relaxed mt-2"
			style="font-size: clamp(0.75rem, 1.1vw, 1.1rem);"
		>
			RELEASED ON JUNE 2026
		</p>
	</div>
{/snippet}

{#snippet StepsTwo()}
	<div
		in:fly={{ x: 20, duration: 500, delay: 200 }}
		out:fade={{ duration: 200 }}
		class="col-start-1 row-start-1 flex items-center justify-center flex-col gap-y-[clamp(1rem,2.5vh,2rem)] text-center w-full py-[clamp(1rem,4vh,4rem)]"
	>
		<div class="flex flex-row items-center justify-center gap-[clamp(0.75rem,2vw,2rem)]">
			<img
				src="tealinux.svg"
				alt="Tealinux Logo"
				style="width: clamp(3.5rem, 7vw, 8rem); height: clamp(3.5rem, 7vw, 8rem);"
			/>
			<h1
				class="text-[#26A768] font-semibold text-5xl md:text-7xl lg:text-8xl xl:text-9xl tracking-tighter leading-none"
			>
				TeaLinuxOS
			</h1>
		</div>

		<p
			class="text-[#E0E0E0] max-w-[clamp(28rem,55vw,56rem)] px-4 leading-relaxed"
			style="font-size: clamp(0.85rem, 1.2vw, 1.15rem);"
		>
			TeaLinuxOS adalah distro Linux turunan Arch Linux yang dikembangkan oleh Dinus Open Source
			Community (DOSCOM) dan kawan-kawan yang berorientasi pemrograman.
		</p>

		<p class="text-[#6A7282]" style="font-size: clamp(0.75rem, 1vw, 0.9rem);">
			Released on December 24, 2024
		</p>

		{@render ChangelogModal()}
	</div>
{/snippet}

{#snippet StepsThree()}
	<div
		in:fly={{ x: 20, duration: 500, delay: 200 }}
		out:fade={{ duration: 200 }}
		class="col-start-1 row-start-1 flex items-center justify-center flex-col text-center w-full max-w-5xl mx-auto py-[clamp(1rem,3vh,3rem)]"
	>
		<div class="flex flex-col gap-y-2 mb-[clamp(1.5rem,3vh,2.5rem)]">
			<h1
				class="bg-linear-to-r from-[#FFFFFF] to-[#99A1AF] bg-clip-text text-transparent font-bold py-1"
				style="font-size: clamp(1.8rem, 4vw, 4rem);"
			>
				Quick Start Guide
			</h1>
			<p class="text-[#6A7282] max-w-xl mx-auto" style="font-size: clamp(0.8rem, 1.1vw, 1.1rem);">
				Essential tools for your TeaLinuxOS experience
			</p>
		</div>

		<div class="grid grid-cols-2 gap-[clamp(0.75rem,1.5vw,1.5rem)] w-full">
			{#each MENU_LISTS as menu, i (i)}
				<Item.Root
					variant="outline"
					class="
						group w-full relative overflow-hidden
						rounded-2xl
						border border-[#00C95033]
						bg-[#0D0D0D]
						bg-linear-to-r from-transparent to-[#00C9501A]
						hover:border-[#00C95066]
						hover:to-[#00C95026]
						transition-all duration-300
						cursor-pointer
						flex items-center gap-[clamp(0.75rem,1.5vw,1.25rem)] text-left
					"
					style="padding: clamp(0.875rem,1.5vh,1.5rem) clamp(1rem,2vw,1.75rem);"
				>
					<Item.Media
						class="
							shrink-0 flex items-center justify-center
							bg-[#094220] border-[#00C9504D] border-2
							rounded-xl
							shadow-[0_0_15px_rgba(0,201,80,0.2)]
						"
						style="width: clamp(2.25rem,3.5vw,3.25rem); height: clamp(2.25rem,3.5vw,3.25rem);"
						variant="icon"
					>
						<menu.icon
							style="width: clamp(1rem,1.5vw,1.5rem); height: clamp(1rem,1.5vw,1.5rem);"
							class="text-[#00C950]"
						/>
					</Item.Media>

					<Item.Content class="flex flex-col items-start justify-center flex-1 min-w-0">
						<Item.Title
							class="text-white font-semibold w-full truncate"
							style="font-size: clamp(0.85rem,1.2vw,1.15rem);"
						>
							{menu.title}
						</Item.Title>
						<Item.Description
							class="text-[#99A1AF] line-clamp-2"
							style="font-size: clamp(0.75rem,1vw,0.9rem);"
						>
							{menu.description}
						</Item.Description>
					</Item.Content>

					<Item.Actions class="shrink-0">
						<ChevronRight
							class="text-[#6A7282] group-hover:text-white group-hover:translate-x-1 transition-transform duration-300"
							style="width: clamp(1rem,1.4vw,1.25rem); height: clamp(1rem,1.4vw,1.25rem);"
						/>
					</Item.Actions>
				</Item.Root>
			{/each}
		</div>
	</div>
{/snippet}

{#snippet ChangelogModal()}
	<Dialog.Root>
		<Dialog.Trigger class="mt-2">
			<Button
				class="backdrop-blur-md border border-[#00C95033] bg-gradient-to-b from-[#0000001A] to-[#26A7681A] shadow-lg
                  text-sm md:text-base px-6 py-2 rounded-full text-white
                  hover:border-[#00C950]
                  hover:from-[#0000004D]
                  hover:to-[#26A7684D]
                  hover:scale-105
                  transition-all duration-300 ease-in-out"
			>
				What's new in TeaLinuxOS
			</Button>
		</Dialog.Trigger>
		<Dialog.Content
			class="min-w-2xl bg-[#0D0D0D] border-[#00C950]/20 border drop-shadow-2xl text-white rounded-lg"
		>
			<Dialog.Header>
				<Dialog.Title>What's new in TeaLinuxOS</Dialog.Title>
			</Dialog.Header>
			<section class="flex flex-col gap-y-4 mt-2">
				{#each CHANGELOG_LISTS as item (item)}
					<div class="flex flex-row gap-x-4">
						<div class="size-6 rounded-full bg-[#26A768]/10 flex items-center justify-center">
							<Check class="text-[#26A768] size-4" />
						</div>
						<p class="text-sm leading-relaxed">{item}</p>
					</div>
				{/each}
			</section>
		</Dialog.Content>
	</Dialog.Root>
{/snippet}
