<script lang="ts">
	import InstallSizeStats from './install-action-bar/InstallSizeStats.svelte';
	import InstallButtons from './install-action-bar/InstallButtons.svelte';
	import ForceRemoveDialog from './install-action-bar/ForceRemoveDialog.svelte';

	interface Props {
		selectedCount: number;
		uninstallCount: number;
		/** Human-readable download size, e.g. "245.3 MiB" */
		totalSize: string;
		/** Human-readable installed-on-disk size, e.g. "812.1 MiB" */
		totalInstallSize?: string;
		/** True while getPackageSizes is in flight */
		sizeLoading?: boolean;
		estTime: string;
		installState: 'idle' | 'installing' | 'success' | 'error';
		installMessage: string;
		uninstallState: 'idle' | 'uninstalling' | 'success' | 'error';
		uninstallMessage: string;
		hasDependencyError: boolean;
		onInstall: () => void;
		onUninstall: () => void;
		onForceUninstall: () => void;
		onCancel: () => void;
	}

	let {
		selectedCount,
		uninstallCount,
		totalSize,
		totalInstallSize = '',
		sizeLoading = false,
		estTime,
		installState,
		installMessage,
		uninstallState,
		uninstallMessage,
		hasDependencyError,
		onInstall,
		onUninstall,
		onForceUninstall,
		onCancel
	}: Props = $props();

	let isProcessing = $derived(installState === 'installing' || uninstallState === 'uninstalling');
	let showForceDialog = $state(false);
</script>

<!-- Bottom Action Bar -->
<div class="w-full max-w-5xl mx-auto px-6 pb-6 pt-2">
	<div
		class="rounded-3xl bg-card/90 backdrop-blur-md border border-border shadow-2xl flex items-center justify-between gap-4 px-8 py-4 max-sm:px-5"
	>
		<!-- Left: size statistics -->
		<InstallSizeStats
			{selectedCount}
			{uninstallCount}
			{totalSize}
			{totalInstallSize}
			{sizeLoading}
			{estTime}
		/>

		<!-- Right: action buttons -->
		<InstallButtons
			{selectedCount}
			{uninstallCount}
			{sizeLoading}
			{installState}
			{installMessage}
			{uninstallState}
			{uninstallMessage}
			{hasDependencyError}
			{isProcessing}
			{onInstall}
			{onUninstall}
			onForceRemoveClick={() => (showForceDialog = true)}
			{onCancel}
		/>
	</div>
</div>

<!-- Force Remove Confirmation Dialog -->
<ForceRemoveDialog
	open={showForceDialog}
	onConfirm={onForceUninstall}
	onClose={() => (showForceDialog = false)}
/>
