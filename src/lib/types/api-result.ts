/**
 * Frontend integration contract aligned with Modularitea tooling.
 */

export type FrontendError =
	| 'PERMISSION_DENIED'
	| 'NETWORK_ERROR'
	| 'INVALID_ARGUMENT'
	| 'COMMAND_FAILED'
	| 'TIMEOUT'
	| 'UNKNOWN_ERROR';

export type ApiResult<T> =
	| { success: true; data: T }
	| { success: false; error: string; code: FrontendError };

export function apiOk<T>(data: T): ApiResult<T> {
	return { success: true, data };
}

export function apiErr(error: string, code: FrontendError = 'UNKNOWN_ERROR'): ApiResult<never> {
	return { success: false, error, code };
}

export function coerceCode(raw: string | undefined | null): FrontendError {
	if (!raw) return 'UNKNOWN_ERROR';
	const upper = raw.toUpperCase();
	const allowed = new Set<FrontendError>([
		'PERMISSION_DENIED',
		'NETWORK_ERROR',
		'INVALID_ARGUMENT',
		'COMMAND_FAILED',
		'TIMEOUT',
		'UNKNOWN_ERROR'
	]);
	return allowed.has(upper as FrontendError) ? (upper as FrontendError) : 'UNKNOWN_ERROR';
}
