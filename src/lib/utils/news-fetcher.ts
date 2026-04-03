import { fetch as reqwest } from '@tauri-apps/plugin-http';
import { XMLParser } from 'fast-xml-parser';
import { decode } from 'html-entities';
import DOMPurify from 'dompurify';

export interface UnifiedNewsItem {
	id: string;
	title: string;
	description: string;
	link: string;
	thumbnail: string;
	pubDate: string;
	creator: string;
}

export type RSS_URL =
	| 'https://www.phoronix.com/rss.php'
	| 'https://fosslinux.com/feed'
	| 'https://itsfoss.com/rss/';

export const fetchRssFeed = async (url: RSS_URL): Promise<UnifiedNewsItem[]> => {
	const response = await reqwest(url, { method: 'GET' });

	if (!response.ok) throw new Error('Failed to Fetch RSS feed');

	const xmlData = await response.text();

	const parser = new XMLParser({
		ignoreAttributes: false,
		attributeNamePrefix: '@_'
	});

	const jsonData = parser.parse(xmlData);

	const rawItems: any = jsonData.rss?.channel?.item;

	const items = Array.isArray(rawItems) ? rawItems : [rawItems];

	return items.map((item): UnifiedNewsItem => {
		const id = typeof item.guid === 'string' ? item.guid : item.guid?.['#text'] || item.link;
		const fallbackImage = 'https://placehold.co/600x400/1e293b/ffffff?text=TealinuxOS&font=montserrat';
		const thumbnail = item['media:content']?.['@_url'] || fallbackImage;
		return {
			id,
			title: decode(item.title || 'No Title'),
			description: DOMPurify.sanitize(item.description || '', {
				ALLOWED_TAGS: ['b', 'i', 'em', 'strong', 'a', 'p', 'br'],
				ALLOWED_ATTR: ['href', 'target', 'rel']
			}),
			link: item.link || '#',
			thumbnail,
			pubDate: item.pubDate || '',
			creator: item['dc:creator'] || 'Unknown Author'
		};
	});
};
