/**
 * installManager.ts — Centralized install/uninstall orchestrator.
 *
 * Manages the lifecycle of all package installs:
 *   1. Calls `install_profile_async` backend command (returns immediately).
 *   2. Subscribes to `install-started`, `install-log`, `install-progress`,
 *      `install-finished`, and `install-failed` Tauri events.
 *   3. Updates both `installStore` and the new package-specific `installStateStore`.
 *   4. Re-syncs states on page load/mount/app-startup via `syncActiveTasks`.
 */

import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { commands, type ProfileInfo, type ActiveInstallTask } from '$lib/commands';
import { installStore } from '$lib/stores/install.svelte';
import { installStateStore } from '$lib/stores/installState.svelte';

// ─── Task ID counter ─────────────────────────────────────────────────────────

let _taskCounter = 0;
function nextId() {
	return `task-${Date.now()}-${++_taskCounter}`;
}

function timestamp() {
	return new Date().toLocaleTimeString('en-GB', { hour12: false });
}

// ─── Event payloads from backend ─────────────────────────────────────────────

type BackendStartedPayload = {
	task_id: string;
};

type BackendLogPayload = {
	task_id: string;
	line: string;
	stream: 'stdout' | 'stderr' | 'system';
	ts: number;
};

type BackendProgressPayload = {
	task_id: string;
	step: string;
	percent: number;
};

type BackendFinishedPayload = {
	task_id: string;
	success: boolean;
	message: string;
};

// ─── Singleton listener state ─────────────────────────────────────────────────

let listenersRegistered = false;
let unlistenFns: UnlistenFn[] = [];

/**
 * Register all global Tauri event listeners.
 * Safe to call multiple times — only registers once.
 */
export async function initInstallListeners(): Promise<void> {
	if (listenersRegistered) return;
	listenersRegistered = true;

	console.log('[installManager] Registering global event listeners');

	const unlistenStarted = await listen<BackendStartedPayload>('install-started', (event) => {
		const { task_id } = event.payload;
		console.log(`[installManager] Event: install-started (task=${task_id})`);
	});

	const unlistenLog = await listen<BackendLogPayload>('install-log', (event) => {
		const { task_id, line, stream } = event.payload;
		// Update individual package store
		installStateStore.appendLog(task_id, line, stream);
		
		// Update main active store if this is the active task
		if (installStore.activeInstall?.id === task_id) {
			installStore.appendInstallLog(line, stream);
		}
	});

	const unlistenProgress = await listen<BackendProgressPayload>(
		'install-progress',
		(event) => {
			const { task_id, step, percent } = event.payload;
			// Update individual package store
			installStateStore.updateProgress(task_id, step, percent);

			// Update main active store if this is the active task
			if (installStore.activeInstall?.id === task_id) {
				installStore.updateInstallProgress(
					step as import('$lib/stores/install.svelte').InstallStep,
					percent
				);
			}
		}
	);

	const unlistenFinished = await listen<BackendFinishedPayload>(
		'install-finished',
		(event) => {
			const { task_id, success, message } = event.payload;
			console.log(`[installManager] Event: install-finished success=${success} (task=${task_id})`);
			
			// Update individual package store
			installStateStore.finishInstall(task_id, success, message);

			// Update main active store if this is the active task
			if (installStore.activeInstall?.id === task_id) {
				installStore.appendInstallLog(
					`[${timestamp()}] ${success ? '✓ Finished' : '✗ Failed'}: ${message}`,
					'system'
				);
				installStore.finishInstall(success, message);
			}
		}
	);

	const unlistenFailed = await listen<BackendFinishedPayload>(
		'install-failed',
		(event) => {
			const { task_id, success, message } = event.payload;
			console.log(`[installManager] Event: install-failed (task=${task_id})`);
			
			// Update individual package store
			installStateStore.finishInstall(task_id, false, message);

			// Update main active store if this is the active task
			if (installStore.activeInstall?.id === task_id) {
				installStore.appendInstallLog(
					`[${timestamp()}] ✗ Failed: ${message}`,
					'system'
				);
				installStore.finishInstall(false, message);
			}
		}
	);

	unlistenFns = [unlistenStarted, unlistenLog, unlistenProgress, unlistenFinished, unlistenFailed];

	// Run recovery sync immediately after setting up listeners
	await syncActiveTasks();
}

/**
 * Re-sync state with the Tauri backend.
 * Queries active tasks from Rust and restores frontend store state.
 */
export async function syncActiveTasks(): Promise<void> {
	console.log('[installManager] Synchronizing active tasks with backend...');
	
	// Query backend active tasks
	await installStateStore.syncWithBackend();

	// If there's an active task in installStateStore but not in installStore, restore it
	const activeTaskId = installStateStore.activeTaskId;
	if (activeTaskId) {
		const activeTask = installStateStore.activeTasks[activeTaskId];
		if (activeTask && (!installStore.activeInstall || installStore.activeInstall.id !== activeTaskId)) {
			console.log(`[installManager] Restoring active install task in main store: ${activeTaskId}`);
			
			// Try to find the profileId from activeTask (can store in profileStates)
			// For recovery, reconstruct InstallTask
			const restoredTask: import('$lib/stores/install.svelte').InstallTask = {
				id: activeTask.task_id,
				label: activeTask.profile_name,
				packages: activeTask.packages,
				services: activeTask.services,
				profileId: activeTask.packages[0] ? activeTask.packages[0] : 'recovered', // fallback
				phase: 'installing',
				step: activeTask.step as any,
				progress: activeTask.percent,
				logs: activeTask.logs.map(l => ({ line: l.line, stream: l.stream as any, ts: Number(l.ts) })),
				message: `Recovered background install for ${activeTask.profile_name}`,
				startedAt: Number(activeTask.started_at),
				finishedAt: null
			};
			
			installStore.restoreActiveInstall(restoredTask);
		}
	}
}

/**
 * Cleanup listeners.
 */
export function cleanupInstallListeners(): void {
	unlistenFns.forEach((fn) => fn());
	unlistenFns = [];
	listenersRegistered = false;
	console.log('[installManager] Listeners cleaned up');
}

// ─── Install profile ──────────────────────────────────────────────────────────

export async function installProfile(
	profile: Pick<ProfileInfo, 'id' | 'name' | 'packages_install' | 'services_enable'>
): Promise<void> {
	if (installStore.isBusy) {
		console.warn('[installManager] Another task is already running.');
		return;
	}

	const packages = profile.packages_install;
	const services = profile.services_enable;

	if (packages.length === 0) return;

	await initInstallListeners();

	const taskId = nextId();

	installStore.startInstall({
		id: taskId,
		label: profile.name,
		packages,
		services,
		profileId: profile.id
	});

	installStore.appendInstallLog(
		`[${timestamp()}] Packages: ${packages.join(', ')}`,
		'system'
	);

	try {
		installStore.appendInstallLog(`[${timestamp()}] Launching background install…`, 'system');
		await commands.installProfileAsync(taskId, profile.id, packages, services);
		installStore.appendInstallLog(
			`[${timestamp()}] Background task started (task=${taskId})`,
			'system'
		);
	} catch (err) {
		const msg = String(err);
		installStore.appendInstallLog(`[${timestamp()}] ERROR launching install: ${msg}`, 'system');
		installStore.finishInstall(false, msg);
	}
}

// ─── Install ad-hoc packages ──────────────────────────────────────────────────

export async function installPackages(
	profileId: string,
	label: string,
	packages: string[],
	services: string[] = []
): Promise<void> {
	if (installStore.isBusy) {
		console.warn('[installManager] Another task is already running.');
		return;
	}
	if (packages.length === 0) return;

	await initInstallListeners();

	const taskId = nextId();

	installStore.startInstall({
		id: taskId,
		label,
		packages,
		services,
		profileId
	});

	installStore.appendInstallLog(`[${timestamp()}] Packages: ${packages.join(', ')}`, 'system');

	try {
		await commands.installProfileAsync(taskId, profileId, packages, services);
		installStore.appendInstallLog(
			`[${timestamp()}] Background task started (task=${taskId})`,
			'system'
		);
	} catch (err) {
		const msg = String(err);
		installStore.appendInstallLog(`[${timestamp()}] ERROR: ${msg}`, 'system');
		installStore.finishInstall(false, msg);
	}
}

// ─── Cancel install ───────────────────────────────────────────────────────────

export async function cancelInstall(): Promise<void> {
	try {
		await commands.cancelInstall();
		if (installStore.activeInstall) {
			installStateStore.cancelInstall(installStore.activeInstall.id);
		}
		installStore.cancelInstall();
	} catch (err) {
		console.warn('[installManager] cancelInstall backend error:', err);
		if (installStore.activeInstall) {
			installStateStore.cancelInstall(installStore.activeInstall.id);
		}
		installStore.cancelInstall();
	}
}

// ─── Uninstall ────────────────────────────────────────────────────────────────

export async function uninstallPackages(
	profileId: string,
	label: string,
	packages: string[],
	services: string[] = [],
	force = false
): Promise<{ success: boolean; hasDependencyError: boolean }> {
	if (installStore.isBusy) {
		console.warn('[installManager] Another task is already running.');
		return { success: false, hasDependencyError: false };
	}
	if (packages.length === 0) return { success: true, hasDependencyError: false };

	installStore.startUninstall({
		id: nextId(),
		label,
		packages,
		services,
		profileId
	});

	installStore.appendUninstallLog(
		`[${timestamp()}] Removing: ${packages.join(', ')} (force=${force})`,
		'system'
	);

	try {
		const result = await commands.removePackages(packages, force);

		result.stdout
			?.split('\n')
			.filter(Boolean)
			.forEach((l) => installStore.appendUninstallLog(l, 'stdout'));
		result.stderr
			?.split('\n')
			.filter(Boolean)
			.forEach((l) => installStore.appendUninstallLog(l, 'stderr'));

		const isDependencyIssue =
			!force &&
			(result.stderr.includes('depend on it') ||
				result.stderr.includes('required by') ||
				result.stderr.includes('Cannot remove'));

		const hasDependencyError = isDependencyIssue && !result.success;

		if (result.success) {
			installStore.finishUninstall(
				true,
				result.stdout || 'Packages removed successfully.',
				false
			);
		} else {
			installStore.finishUninstall(
				false,
				result.stderr || 'Uninstall failed.',
				hasDependencyError
			);
		}

		return { success: result.success, hasDependencyError };
	} catch (err) {
		const msg = String(err);
		installStore.appendUninstallLog(`[${timestamp()}] ERROR: ${msg}`, 'system');
		installStore.finishUninstall(false, msg, false);
		return { success: false, hasDependencyError: false };
	}
}
