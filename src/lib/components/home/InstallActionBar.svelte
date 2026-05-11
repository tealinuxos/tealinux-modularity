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
<div
	class="w-800px sticky bottom-0 z-50 -mx-4 -mb-4 bg-background/95 backdrop-blur-sm border-t border-border px-8 pt-4 pb-8 flex items-center justify-between mt-auto"
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

<!-- Force Remove Confirmation Dialog -->
<ForceRemoveDialog
	open={showForceDialog}
	onConfirm={onForceUninstall}
	onClose={() => (showForceDialog = false)}
/>
