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

<footer class={cn('flex flex-col')}>
	<div class="mx-auto mb-12">
		<StepIndicator {currentStep} {totalSteps} />
	</div>
	<div
		class={cn('flex flex-row items-center', currentStep === 0 ? 'justify-end' : 'justify-between')}
	>
		{#if currentStep > 0}
			<Button variant="link" size="lg" class="flex flex-row gap-x-2 text-white" onclick={onBack}>
				<ChevronLeft />
				Back
			</Button>
		{/if}

		<Button size="lg" class="flex flex-row gap-x-2" onclick={onNext}>
			Next
			<ChevronRight />
		</Button>
	</div>
</footer>
