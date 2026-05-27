import type { CpuProfile, DnsProvider } from '$lib/commands';
import { browser } from '$app/environment';

type SettingsState = {
	cpuGovernor: CpuProfile | null;
	dnsProvider: DnsProvider | null;
	swapEnabled: boolean | null;
};

const DEFAULT_SETTINGS: SettingsState = {
	cpuGovernor: null,
	dnsProvider: null,
	swapEnabled: null
};

function createSettingsStore() {
	const state = $state<SettingsState>({ ...DEFAULT_SETTINGS });

	return {
		get cpuGovernor() {
			return state.cpuGovernor;
		},
		set cpuGovernor(val: CpuProfile | null) {
			state.cpuGovernor = val;
		},

		get dnsProvider() {
			return state.dnsProvider;
		},
		set dnsProvider(val: DnsProvider | null) {
			state.dnsProvider = val;
		},

		get swapEnabled() {
			return state.swapEnabled;
		},
		set swapEnabled(val: boolean | null) {
			state.swapEnabled = val;
		}
	};
}

export const settingsState = createSettingsStore();

type AppMetaState = {
	lastCacheCleaned: number | null;
};

const DEFAULT_META: AppMetaState = {
	lastCacheCleaned: null
};

const META_KEY = 'tealinux_app_meta';

function createAppMetaStore() {
	let state = $state<AppMetaState>({ ...DEFAULT_META });

	if (browser) {
		const saved = localStorage.getItem(META_KEY);
		if (saved) {
			try {
				state = { ...DEFAULT_META, ...JSON.parse(saved) };
			} catch (e) {
				console.error('Failed to parse app meta from localStorage', e);
			}
		}

		$effect.root(() => {
			$effect(() => {
				localStorage.setItem(META_KEY, JSON.stringify(state));
			});
		});
	}

	return {
		get lastCacheCleaned() {
			return state.lastCacheCleaned;
		},
		set lastCacheCleaned(val: number | null) {
			state.lastCacheCleaned = val;
		}
	};
}

export const appMetaState = createAppMetaStore();
