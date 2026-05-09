import { commands, type ParsedNewsItemDto } from '$lib/commands';
import { coerceCode } from '$lib/types/api-result';
import { apiErr, apiOk, type ApiResult } from '$lib/types/api-result';
import type { ParsedNewsItem } from '$lib/types/news';
import { asApiFailure } from '$lib/utils/normalize-invoke-error';
import { promiseWithTimeout } from '$lib/utils/with-timeout';

const REMOTE_FETCH_TIMEOUT_MS = 15_000;
const LOCAL_CACHE_MS = 60 * 60 * 1000;
const STORAGE_KEY = 'modularitea.news.blackbox.v1';

type Stored = {
	at: number;
	items: ParsedNewsItem[];
};

function dtoToParsed(d: ParsedNewsItemDto): ParsedNewsItem {
	return {
		url: d.url,
		title: d.title,
		descriptive: d.descriptive,
		thumbnail: d.thumbnail ?? null
	};
}

function readLocalCache(): ParsedNewsItem[] | null {
	if (typeof localStorage === 'undefined') return null;
	try {
		const raw = localStorage.getItem(STORAGE_KEY);
		if (!raw) return null;
		const o = JSON.parse(raw) as Stored;
		if (!o?.items || !Number.isFinite(o.at)) return null;
		if (Date.now() - o.at > LOCAL_CACHE_MS) return null;
		return o.items;
	} catch {
		return null;
	}
}

function writeLocalCache(items: ParsedNewsItem[]) {
	try {
		const payload: Stored = { at: Date.now(), items };
		localStorage.setItem(STORAGE_KEY, JSON.stringify(payload));
	} catch {
		/* quota / SSR */
	}
}

/**
 * Loads combined Modularitea-libs RSS via Tauri (`NewsParser.blackbox_fetcher`).
 * Optional 1h browser cache for instant paint; use `forceRefresh` to bypass.
 */
export async function fetchNews(options?: { forceRefresh?: boolean }): Promise<ApiResult<ParsedNewsItem[]>> {
	const force = Boolean(options?.forceRefresh);
	if (!force) {
		const cached = readLocalCache();
		if (cached?.length) return apiOk(cached);
	}

	try {
		const raw = await promiseWithTimeout(REMOTE_FETCH_TIMEOUT_MS, commands.fetchParsedNews(force));
		if (raw.success && raw.data) {
			const items = raw.data.map(dtoToParsed);
			writeLocalCache(items);
			return apiOk(items);
		}
		return apiErr(raw.error ?? 'Failed to load news', coerceCode(raw.code));
	} catch (e) {
		if (e instanceof Error && e.message === 'TIMEOUT') {
			return apiErr('News request timed out', 'TIMEOUT');
		}
		return asApiFailure(e);
	}
}
