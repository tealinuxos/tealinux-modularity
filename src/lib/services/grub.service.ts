import { commands } from '$lib/commands';
import type { ThemeManifest } from '$lib/types/grub';
import { coerceCode } from '$lib/types/api-result';
import { apiErr, apiOk, type ApiResult } from '$lib/types/api-result';
import { promiseWithTimeout } from '$lib/utils/with-timeout';
import { asApiFailure } from '$lib/utils/normalize-invoke-error';

const APPLY_TIMEOUT_MS = 30_000;
const LIST_TIMEOUT_MS = 30_000;

export async function fetchThemes(): Promise<ApiResult<ThemeManifest[]>> {
	try {
		const list = await promiseWithTimeout(LIST_TIMEOUT_MS, commands.getGrubThemes());
		const sorted = [...list].sort((a, b) => a.name.localeCompare(b.name));
		return apiOk(sorted);
	} catch (e) {
		if (e instanceof Error && e.message === 'TIMEOUT') {
			return apiErr('Loading themes timed out', 'TIMEOUT');
		}
		return asApiFailure(e);
	}
}

export async function applyTheme(themeName: string): Promise<ApiResult<void>> {
	try {
		const raw = await promiseWithTimeout(APPLY_TIMEOUT_MS, commands.setGrubTheme(themeName));
		if (raw.success) return apiOk(undefined);
		return apiErr(raw.error ?? 'Failed to apply theme', coerceCode(raw.code));
	} catch (e) {
		if (e instanceof Error && e.message === 'TIMEOUT') {
			return apiErr('Apply theme timed out', 'TIMEOUT');
		}
		return asApiFailure(e);
	}
}
