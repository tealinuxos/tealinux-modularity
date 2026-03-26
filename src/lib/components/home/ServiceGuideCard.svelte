<script lang="ts">
	import { Terminal } from '@lucide/svelte';
	import CopyCodeBlock from './CopyCodeBlock.svelte';

	type ServiceCommand = {
		label: string;
		cmd: string;
		type: 'setup' | 'maintenance';
	};

	type ServiceGuide = {
		description: string;
		commands: ServiceCommand[];
	};

	interface Props {
		svc: string;
		guide: ServiceGuide | undefined;
		copiedCmd: string | null;
		onCopy: (cmd: string) => void;
	}

	let { svc, guide, copiedCmd, onCopy }: Props = $props();
</script>

<div
	class="overflow-hidden rounded-[0.875rem] border border-border bg-card transition-all duration-200 hover:border-border/80 hover:shadow-[0_4px_24px_-4px_var(--foreground)/0.04,0_1px_4px_var(--foreground)/0.02]"
>
	<!-- Card Header -->
	<div class="flex items-start gap-3.5 px-5 pt-5 pb-3">
		<div
			class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-blue-500/10 text-blue-500"
		>
			<Terminal class="h-5 w-5" />
		</div>
		<div class="flex min-w-0 flex-col gap-0.5">
			<h4 class="m-0 text-[0.95rem] font-bold capitalize tracking-tight text-foreground">
				{svc}
			</h4>
			{#if guide}
				<p class="m-0 text-[0.78rem] leading-relaxed text-muted-foreground">
					{guide.description}
				</p>
			{/if}
		</div>
	</div>

	<!-- Enable command -->
	<div class="mx-5 border-b border-border/50 pb-3.5">
		<span class="mb-1.5 block font-mono text-[0.68rem] tracking-wide text-muted-foreground/60">
			# Enable and start the service
		</span>
		<CopyCodeBlock cmd={`systemctl enable --now ${svc}`} {copiedCmd} {onCopy} />
	</div>

	<!-- Stepper Commands -->
	{#if guide}
		<div class="flex flex-col px-5 pt-4 pb-5">
			{#each guide.commands as step, i}
				{@const isLast = i === guide.commands.length - 1}
				<div class="relative flex gap-4">
					<!-- Track (dot + line) -->
					<div class="flex shrink-0 flex-col items-center pt-1.5" style="width: 1.125rem;">
						<div
							class="relative z-10 h-2 w-2 shrink-0 rounded-full border-2 transition-all duration-200
								{step.type === 'setup'
								? 'border-green-400/25 bg-green-400/70'
								: 'border-blue-500/20 bg-blue-500/60'}"
						></div>
						{#if !isLast}
							<div class="my-1 flex-1 bg-border/50" style="width: 1.5px;"></div>
						{/if}
					</div>

					<!-- Content -->
					<div class="flex min-w-0 flex-1 flex-col gap-1.5 {isLast ? '' : 'pb-5'}">
						<div class="flex flex-wrap items-center gap-2">
							<span class="text-[0.8rem] font-semibold tracking-tight text-foreground">
								{step.label}
							</span>
							<span
								class="inline-flex items-center rounded-full px-2 py-px text-[0.62rem] font-semibold uppercase leading-relaxed tracking-wider
									{step.type === 'setup'
									? 'border border-green-400/20 bg-green-400/12 text-green-400'
									: 'border border-blue-500/15 bg-blue-500/10 text-blue-500'}"
							>
								{step.type === 'setup' ? 'Setup' : 'Maintenance'}
							</span>
						</div>
						<CopyCodeBlock cmd={step.cmd} {copiedCmd} {onCopy} />
					</div>
				</div>
			{/each}
		</div>
	{:else}
		<!-- Fallback -->
		<div class="px-5 pt-4 pb-5">
			<p class="m-0 text-[0.78rem] italic leading-relaxed text-muted-foreground">
				No specific guide available. The service will be enabled automatically during installation.
			</p>
		</div>
	{/if}
</div>
