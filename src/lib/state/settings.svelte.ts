import type { CpuProfile, DnsProvider } from '$lib/commands';
import { browser } from '$app/environment';

type SettingsState = {
	cpuGovernor: CpuProfile;
	dnsProvider: DnsProvider;
	swapEnabled: boolean;
	lastCacheCleaned: number | null;
};

const DEFAULT_SETTINGS: SettingsState = {
	cpuGovernor: 'performance',
	dnsProvider: 'cloudflare',
	swapEnabled: false,
	lastCacheCleaned: null
};

function createSettingsStore() {
	let state = $state<SettingsState>(DEFAULT_SETTINGS);

	if (browser) {
		const saved = localStorage.getItem('tealinux_settings');
		if (saved) {
			try {
				state = { ...DEFAULT_SETTINGS, ...JSON.parse(saved) };
			} catch (e) {
				console.error('Failed to parse settings from localStorage', e);
			}
		}

		$effect.root(() => {
			$effect(() => {
				localStorage.setItem('tealinux_settings', JSON.stringify(state));
			});
		});
	}

	return {
		get cpuGovernor() { return state.cpuGovernor; },
		set cpuGovernor(val: CpuProfile) { state.cpuGovernor = val; },

		get dnsProvider() { return state.dnsProvider; },
		set dnsProvider(val: DnsProvider) { state.dnsProvider = val; },

		get swapEnabled() { return state.swapEnabled; },
		set swapEnabled(val: boolean) { state.swapEnabled = val; },

		get lastCacheCleaned() { return state.lastCacheCleaned; },
		set lastCacheCleaned(val: number | null) { state.lastCacheCleaned = val; }
	};
}

export const settingsState = createSettingsStore();
