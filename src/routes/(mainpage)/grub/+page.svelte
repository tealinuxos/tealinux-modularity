<script lang="ts">
	import { SlidersHorizontal } from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card/index';
	import * as Carousel from '$lib/components/ui/carousel/index.js';
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
					class="w-full h-full rounded-md border flex items-center justify-center overflow-hidden"
				>
					<img
						src="https://placehold.co/600x337?text=bootloader1"
						alt="Theme 1"
						class="w-full h-full object-cover"
					/>
				</div>
			</section>

			<section class="flex flex-col shrink-0">
				<p class="text-[#99A1AF] mb-2">Choose A Theme</p>
				<Carousel.Root
					opts={{
						align: 'start',
						skipSnaps: true
					}}
					class="w-full"
				>
					<Carousel.Content>
						{#each Array(10), i}
							<Carousel.Item class="basis-[85%] sm:basis-1/2 md:basis-1/3 lg:basis-1/4">
								<div class="p-1">
									<Card.Root
										class="flex aspect-video items-center justify-center p-0 overflow-hidden rounded-xl border-2 border-transparent hover:border-primary cursor-pointer transition-all"
									>
										<img
											src="https://placehold.co/600x337?text=Theme+{i + 1}"
											alt="Theme {i + 1}"
											class="w-full h-full object-cover"
										/>
									</Card.Root>
								</div>
							</Carousel.Item>
						{/each}
					</Carousel.Content>
				</Carousel.Root>
			</section>
		</Card.Content>

		<Card.Footer class="pb-4 px-4 pt-0 shrink-0">
			<Button class="w-full">Apply Now</Button>
		</Card.Footer>
	</Card.Root>
</main>
