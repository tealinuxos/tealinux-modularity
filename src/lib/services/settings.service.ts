import { commands } from '$lib/commands';
import { coerceCode } from '$lib/types/api-result';
import { apiErr, apiOk, type ApiResult } from '$lib/types/api-result';
import type { CpuProfile, DnsProvider } from '$lib/types/settings';
import { isCpuProfile, isDnsProvider } from '$lib/utils/safe-args';
import { asApiFailure } from '$lib/utils/normalize-invoke-error';
import { promiseWithTimeout } from '$lib/utils/with-timeout';

const SHORT_MS = 5000;
const LONG_MS = 30000;

export async function listMirrorCountries(): Promise<ApiResult<string[]>> {
	try {
		const list = await promiseWithTimeout(SHORT_MS, commands.mirrorReflectorCountryList());
		return apiOk(list);
	} catch (e) {
		if (e instanceof Error && e.message === 'TIMEOUT') return apiErr('Timed out', 'TIMEOUT');
		return asApiFailure(e);
	}
}

export async function refreshMirror(country: string): Promise<ApiResult<string>> {
	try {
		const raw = await promiseWithTimeout(LONG_MS, commands.settingsRefreshMirror(country));
		if (raw.success && raw.data !== undefined) return apiOk(raw.data);
		return apiErr(raw.error ?? 'Mirror refresh failed', coerceCode(raw.code));
	} catch (e) {
		if (e instanceof Error && e.message === 'TIMEOUT') return apiErr('Timed out', 'TIMEOUT');
		return asApiFailure(e);
	}
}

export async function changeDns(provider: DnsProvider): Promise<ApiResult<void>> {
	if (!isDnsProvider(provider)) {
		return apiErr('Invalid DNS provider', 'INVALID_ARGUMENT');
	}
	try {
		const raw = await promiseWithTimeout(SHORT_MS, commands.settingsChangeDns(provider));
		if (raw.success) return apiOk(undefined);
		return apiErr(raw.error ?? 'DNS change failed', coerceCode(raw.code));
	} catch (e) {
		if (e instanceof Error && e.message === 'TIMEOUT') return apiErr('Timed out', 'TIMEOUT');
		return asApiFailure(e);
	}
}

export async function readDnsSummary(): Promise<ApiResult<string>> {
	try {
		const raw = await promiseWithTimeout(SHORT_MS, commands.settingsDnsStatusLine());
		if (raw.success && raw.data !== undefined) return apiOk(raw.data);
		return apiErr(raw.error ?? 'Could not read resolv.conf', coerceCode(raw.code));
	} catch (e) {
		return asApiFailure(e);
	}
}

export async function toggleSwap(enabled: boolean): Promise<ApiResult<void>> {
	try {
		const raw = await promiseWithTimeout(LONG_MS, commands.settingsToggleSwap(enabled));
		if (raw.success) return apiOk(undefined);
		return apiErr(raw.error ?? 'Swap toggle failed', coerceCode(raw.code));
	} catch (e) {
		if (e instanceof Error && e.message === 'TIMEOUT') return apiErr('Timed out', 'TIMEOUT');
		return asApiFailure(e);
	}
}

export async function swapEnabledFromConfig(): Promise<ApiResult<boolean>> {
	try {
		const raw = await promiseWithTimeout(SHORT_MS, commands.settingsSwapEnabledState());
		if (raw.success && raw.data !== undefined) return apiOk(raw.data);
		return apiErr(raw.error ?? 'Swap state unreadable', coerceCode(raw.code));
	} catch (e) {
		return asApiFailure(e);
	}
}

export async function cleanPackageCache(): Promise<ApiResult<boolean>> {
	try {
		const raw = await promiseWithTimeout(LONG_MS, commands.settingsCleanPackageCache());
		if (raw.success && raw.data !== undefined) return apiOk(raw.data);
		return apiErr(raw.error ?? 'Cache clean failed', coerceCode(raw.code));
	} catch (e) {
		if (e instanceof Error && e.message === 'TIMEOUT') return apiErr('Timed out', 'TIMEOUT');
		return asApiFailure(e);
	}
}

export async function setCpuProfile(profile: CpuProfile): Promise<ApiResult<void>> {
	if (!isCpuProfile(profile)) {
		return apiErr('Invalid CPU profile', 'INVALID_ARGUMENT');
	}
	try {
		const raw = await promiseWithTimeout(SHORT_MS, commands.settingsSetCpuProfile(profile));
		if (raw.success) return apiOk(undefined);
		return apiErr(raw.error ?? 'CPU profile failed', coerceCode(raw.code));
	} catch (e) {
		if (e instanceof Error && e.message === 'TIMEOUT') return apiErr('Timed out', 'TIMEOUT');
		return asApiFailure(e);
	}
}

export async function readCpuGovernor(): Promise<ApiResult<string>> {
	try {
		const raw = await promiseWithTimeout(SHORT_MS, commands.settingsCpuGovernorLine());
		if (raw.success && raw.data !== undefined) return apiOk(raw.data);
		return apiErr(raw.error ?? 'Could not read governor', coerceCode(raw.code));
	} catch (e) {
		return asApiFailure(e);
	}
}
