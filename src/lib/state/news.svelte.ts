import type { RSS_URL } from '$lib/utils/news-fetcher';

export const newsState = $state<{ provider: RSS_URL }>({
	provider: 'https://itsfoss.com/rss/'
});
