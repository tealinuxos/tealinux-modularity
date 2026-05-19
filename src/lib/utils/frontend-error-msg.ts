import type { FrontendError } from '$lib/types/api-result';

/** User-facing fallback text for normalized error codes (no stack traces). */
export function briefErrorMessage(code: FrontendError | undefined, detail?: string): string {
	switch (code) {
		case 'PERMISSION_DENIED':
			return detail?.trim()
				? `Permission denied: ${detail}`
				: 'Permission denied or the authorization prompt was dismissed.';
		case 'NETWORK_ERROR':
			return detail?.trim()
				? `Network issue: ${detail}`
				: 'A network error occurred.';
		case 'INVALID_ARGUMENT':
			return detail?.trim() ? detail : 'Invalid argument.';
		case 'COMMAND_FAILED':
			return detail?.trim() ? detail : 'The backend command failed.';
		case 'TIMEOUT':
			return 'The operation took too long and was stopped.';
		default:
			return detail?.trim() ? detail : 'Something went wrong.';
	}
}
