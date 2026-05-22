<script lang="ts">
	import { Button } from '$lib/components/ui/button/';
	import { cn } from '$lib/utils';
	import { ChevronLeft, ChevronRight } from '@lucide/svelte';
	import StepIndicator from './StepIndicator.svelte';

	interface Props {
		totalSteps: number;
		currentStep: number;
		onBack: () => void;
		onNext: () => void;
	}

	let { onBack, onNext, currentStep, totalSteps }: Props = $props();
</script>

<footer class={cn('flex flex-col w-full max-w-7xl mx-auto px-4')}>
	<div class="mx-auto mb-6 md:mb-12">
		<StepIndicator {currentStep} {totalSteps} />
	</div>
	<div
		class={cn(
			'flex flex-row items-center w-full pb-4',
			currentStep === 0 ? 'justify-end' : 'justify-between'
		)}
	>
		{#if currentStep > 0}
			<Button
				variant="link"
				size="lg"
				class="flex flex-row gap-x-2 text-white hover:text-[#54CD4C] transition-colors"
				onclick={onBack}
			>
				<ChevronLeft class="size-5" />
				Back
			</Button>
		{/if}

		<Button
			size="lg"
			class="flex flex-row gap-x-2 bg-[#54CD4C] hover:bg-[#3F9A39] text-black font-semibold px-8"
			onclick={onNext}
		>
			Next
			<ChevronRight class="size-5" />
		</Button>
	</div>
</footer>
