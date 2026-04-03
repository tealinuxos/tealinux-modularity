export type ItsFOSSTypes = Root2[];

interface Root2 {
	title: string;
	description: string;
	link: string;
	guid: Guid;
	category: string;
	'dc:creator': string;
	pubDate: string;
	'media:content': MediaContent;
	'content:encoded': string;
}

interface Guid {
	'#text': string;
	'@_isPermaLink': string;
}

interface MediaContent {
	'@_url': string;
	'@_medium': string;
}
