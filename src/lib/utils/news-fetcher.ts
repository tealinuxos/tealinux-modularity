import { fetch as reqwest } from '@tauri-apps/plugin-http';
import { XMLParser } from 'fast-xml-parser';

type PHORONIX_RSS_URL = 'https://www.phoronix.com/rss.php';
type FOSSLINUX_RSS_URL = 'https://fosslinux.com/feed';
type ITSFOSS_RSS_URL = 'https://itsfoss.com/rss/';

type RSS_URL = PHORONIX_RSS_URL | FOSSLINUX_RSS_URL | ITSFOSS_RSS_URL;

export const fetchRssFeed = async (RSS_URL: RSS_URL) => {
	const response = await reqwest(RSS_URL, {
		method: 'GET'
	});

	if (!response.ok) throw new Error('Failed to Fetch RSS feed');

	const xmlData = await response.text();

	const parser = new XMLParser({
		ignoreAttributes: false,
		attributeNamePrefix: '@_'
	});

	const jsonData = parser.parse(xmlData);

	return jsonData.rss.channel.item;
};
