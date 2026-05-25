/**
 * installState.svelte.ts — Global package-level install state registry.
 *
 * Tracks the installation state of individual packages in real-time.
 * Used by package cards and badges to display progress spinners,
 * loading animations, and disable buttons while an installation is active.
 *
 * Synchronizes with the Tauri backend on startup and page load/mount.
 */

import { commands, type ActiveInstallTask } from '$lib/commands';

export type PackageInstallStatus = 'idle' | 'queued' | 'installing' | 'success' | 'error' | 'cancelled';

export interface PackageInstallDetail {
	status: PackageInstallStatus;
	progress: number;
	startedAt: number;
	logs: { line: string; stream: 'stdout' | 'stderr' | 'system'; ts: number }[];
	taskId: string;
}

class InstallStateStore {
	// Global map: package_name -> install details
	packages = $state<Record<string, PackageInstallDetail>>({});

	// Active backend tasks: task_id -> task details
	activeTasks = $state<Record<string, ActiveInstallTask>>({});

	// Currently focused task ID for detail log views / overlays
	activeTaskId = $state<string | null>(null);

	// Install queue of package names
	queue = $state<string[]>([]);

	constructor() {
		// Constructor is run on startup (singleton)
	}

	// ─── Query methods ────────────────────────────────────────────────────────

	isInstalling(pkgName: string): boolean {
		return this.packages[pkgName]?.status === 'installing';
	}

	isQueued(pkgName: string): boolean {
		return this.packages[pkgName]?.status === 'queued';
	}

	getProgress(pkgName: string): number {
		return this.packages[pkgName]?.progress ?? 0;
	}

	getStatus(pkgName: string): PackageInstallStatus {
		return this.packages[pkgName]?.status ?? 'idle';
	}

	getLogs(pkgName: string) {
		return this.packages[pkgName]?.logs ?? [];
	}

	// ─── Mutation methods ─────────────────────────────────────────────────────

	setPackageStatus(pkgName: string, status: PackageInstallStatus, details?: Partial<PackageInstallDetail>) {
		if (!this.packages[pkgName]) {
			this.packages[pkgName] = {
				status: 'idle',
				progress: 0,
				startedAt: Date.now(),
				logs: [],
				taskId: ''
			};
		}
		this.packages[pkgName].status = status;
		if (details) {
			if (details.progress !== undefined) this.packages[pkgName].progress = details.progress;
			if (details.startedAt !== undefined) this.packages[pkgName].startedAt = details.startedAt;
			if (details.logs !== undefined) this.packages[pkgName].logs = details.logs;
			if (details.taskId !== undefined) this.packages[pkgName].taskId = details.taskId;
		}
	}

	registerInstall(taskId: string, profileName: string, packages: string[], services: string[]) {
		this.activeTaskId = taskId;

		// Set package states
		for (const pkg of packages) {
			this.setPackageStatus(pkg, 'installing', {
				progress: 0,
				startedAt: Date.now(),
				taskId,
				logs: []
			});
		}

		// Save active task details with correct types (bigint for backend structure representation)
		this.activeTasks[taskId] = {
			task_id: taskId,
			profile_name: profileName,
			packages,
			services,
			step: 'idle',
			percent: 0,
			started_at: BigInt(Date.now()),
			logs: []
		};
	}

	appendLog(taskId: string, line: string, stream: 'stdout' | 'stderr' | 'system') {
		const task = this.activeTasks[taskId];
		if (task) {
			// Push with bigint ts for backend tasks compatibility
			const backendLog = { line, stream, ts: BigInt(Date.now()) };
			task.logs.push(backendLog);
			if (task.logs.length > 500) task.logs.shift();

			// Push with number ts for frontend packages logs compatibility
			const frontendLog = { line, stream, ts: Date.now() };
			for (const pkg of task.packages) {
				const pkgState = this.packages[pkg];
				if (pkgState && pkgState.taskId === taskId) {
					pkgState.logs.push(frontendLog);
					if (pkgState.logs.length > 500) pkgState.logs.shift();
				}
			}
		}
	}

	updateProgress(taskId: string, step: string, percent: number) {
		const task = this.activeTasks[taskId];
		if (task) {
			task.step = step;
			task.percent = percent;

			// Update individual packages
			for (const pkg of task.packages) {
				const pkgState = this.packages[pkg];
				if (pkgState && pkgState.taskId === taskId) {
					pkgState.progress = percent;
					pkgState.status = 'installing';
				}
			}
		}
	}

	finishInstall(taskId: string, success: boolean, message: string) {
		const task = this.activeTasks[taskId];
		if (task) {
			const finalStatus = success ? 'success' : 'error';
			for (const pkg of task.packages) {
				const pkgState = this.packages[pkg];
				if (pkgState && pkgState.taskId === taskId) {
					pkgState.status = finalStatus;
					pkgState.progress = success ? 100 : pkgState.progress;
				}
			}
			delete this.activeTasks[taskId];
			if (this.activeTaskId === taskId) {
				// Keep active task displayed for 8s so user can read log final result, then clear
				setTimeout(() => {
					if (this.activeTaskId === taskId) {
						this.activeTaskId = null;
					}
				}, 8000);
			}
		}
	}

	cancelInstall(taskId: string) {
		const task = this.activeTasks[taskId];
		if (task) {
			for (const pkg of task.packages) {
				const pkgState = this.packages[pkg];
				if (pkgState && pkgState.taskId === taskId) {
					pkgState.status = 'cancelled';
				}
			}
			delete this.activeTasks[taskId];
			if (this.activeTaskId === taskId) {
				this.activeTaskId = null;
			}
		}
	}

	// ─── Recovery/Sync ────────────────────────────────────────────────────────

	async syncWithBackend() {
		try {
			console.log('[installStateStore] Syncing with backend active tasks...');
			const active = await commands.getActiveInstalls();
			console.log('[installStateStore] Active backend tasks response:', active);

			// Map of active task IDs
			const activeTaskIds = new Set(active.map((t) => t.task_id));

			// Remove local active tasks that are no longer running on backend
			for (const id in this.activeTasks) {
				if (!activeTaskIds.has(id)) {
					delete this.activeTasks[id];
				}
			}

			// Restore states for all active backend tasks
			for (const task of active) {
				this.activeTasks[task.task_id] = {
					task_id: task.task_id,
					profile_name: task.profile_name,
					packages: task.packages,
					services: task.services,
					step: task.step,
					percent: task.percent,
					started_at: task.started_at,
					logs: task.logs
				};

				this.activeTaskId = task.task_id;

				// Restore statuses for individual packages
				for (const pkg of task.packages) {
					this.setPackageStatus(pkg, 'installing', {
						progress: task.percent,
						startedAt: Number(task.started_at),
						taskId: task.task_id,
						logs: task.logs.map((l) => ({
							line: l.line,
							stream: l.stream as 'stdout' | 'stderr' | 'system',
							ts: Number(l.ts)
						}))
					});
				}
			}
		} catch (e) {
			console.error('[installStateStore] Sync failed:', e);
		}
	}
}

export const installStateStore = new InstallStateStore();
