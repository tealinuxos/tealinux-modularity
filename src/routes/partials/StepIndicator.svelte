<script lang="ts">
	interface Props {
		currentStep: number;
		totalSteps: number;
	}
	let { currentStep, totalSteps }: Props = $props();
</script>

<div class="flex items-center gap-3">
	{#each Array(totalSteps), i (i)}
		<div
			class="h-3 rounded-full transition-all duration-500 ease-in-out
      {i === currentStep
				? 'w-28 bg-primary'
				: 'w-3 bg-gray-600 hover:bg-gray-500'}"
		></div>
	{/each}
</div>
