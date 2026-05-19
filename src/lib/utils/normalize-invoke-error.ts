import { coerceCode } from '$lib/types/api-result';
import type { ApiResult } from '$lib/types/api-result';
import { apiErr } from '$lib/types/api-result';

/** Map arbitrary thrown values (often Tauri / plugin failures) to `ApiResult`. */
export function asApiFailure<T>(err: unknown): ApiResult<T> {
	if (err instanceof Error) {
		const msg = err.message;
		if (msg === 'TIMEOUT') {
			return apiErr('Operation timed out', 'TIMEOUT');
		}
		if (/denied|polkit|not authorized|cancell?ed/i.test(msg)) {
			return apiErr(msg, 'PERMISSION_DENIED');
		}
		if (/network|timed out/i.test(msg)) {
			return apiErr(msg, 'NETWORK_ERROR');
		}
		return apiErr(msg, coerceCode(undefined));
	}
	if (typeof err === 'string') {
		return apiErr(err, coerceCode(undefined));
	}
	return apiErr('Unknown error', 'UNKNOWN_ERROR');
}
