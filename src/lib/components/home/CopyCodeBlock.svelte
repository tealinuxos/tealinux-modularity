<script lang="ts">
	import { Copy, Check } from 'lucide-svelte';

	interface Props {
		cmd: string;
		copiedCmd: string | null;
		onCopy: (cmd: string) => void;
	}

	let { cmd, copiedCmd, onCopy }: Props = $props();

	let isCopied = $derived(copiedCmd === cmd);
</script>

<div
	class="flex items-center justify-between gap-3 rounded-lg border border-white/8 bg-[#0a0a0a] px-3 py-2.5 transition-colors hover:border-white/14"
>
	<code class="min-w-0 truncate font-mono text-[0.78rem] leading-relaxed text-slate-200">
		{cmd}
	</code>
	<button
		class="inline-flex shrink-0 cursor-pointer items-center gap-1 whitespace-nowrap rounded-md border px-2 py-1 font-sans text-[0.68rem] font-medium transition-all
			{isCopied
			? 'border-green-400/30 bg-green-400/8 text-green-400'
			: 'border-white/10 bg-white/5 text-slate-400 hover:border-white/18 hover:bg-white/10 hover:text-slate-200'}"
		onclick={() => onCopy(cmd)}
	>
		{#if isCopied}
			<Check class="h-3.5 w-3.5" />
			<span>Copied!</span>
		{:else}
			<Copy class="h-3.5 w-3.5" />
			<span>Copy</span>
		{/if}
	</button>
</div>
