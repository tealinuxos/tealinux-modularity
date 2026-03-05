import type { FOSSLinuxTypes } from '$lib/types/news/fosslinux';
import type { ItsFOSSTypes } from '$lib/types/news/itsfoss';
import type { PhoronixTypes } from '$lib/types/news/phoronix';
import { fetch as reqwest } from '@tauri-apps/plugin-http';
import { XMLParser } from 'fast-xml-parser';

interface RssTypeMap {
	'https://www.phoronix.com/rss.php': PhoronixTypes[];
	'https://fosslinux.com/feed': FOSSLinuxTypes[];
	'https://itsfoss.com/rss/': ItsFOSSTypes[];
}

type RSS_URL = keyof RssTypeMap;

export const fetchRssFeed = async <T extends RSS_URL>(url: T): Promise<RssTypeMap[T]> => {
	const response = await reqwest(url, {
		method: 'GET'
	});

	if (!response.ok) throw new Error('Failed to Fetch RSS feed');

	const xmlData = await response.text();

	const parser = new XMLParser({
		ignoreAttributes: false,
		attributeNamePrefix: '@_'
	});

	const jsonData = parser.parse(xmlData);

	return jsonData.rss.channel.item as RssTypeMap[T];
};