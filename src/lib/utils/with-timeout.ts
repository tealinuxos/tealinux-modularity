/** Race `promise` against a timeout; rejects with `Error('TIMEOUT')` on expiry. */
export async function promiseWithTimeout<T>(ms: number, promise: Promise<T>): Promise<T> {
	let id: ReturnType<typeof setTimeout> | undefined;
	const timeoutP = new Promise<never>((_, rej) => {
		id = setTimeout(() => rej(new Error('TIMEOUT')), ms);
	});
	try {
		return await Promise.race([promise, timeoutP]);
	} finally {
		if (id !== undefined) clearTimeout(id);
	}
}
