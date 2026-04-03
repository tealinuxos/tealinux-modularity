export type FOSSLinuxTypes = Root2[];

interface Root2 {
	title: string;
	link: string;
	comments: string;
	'dc:creator': string;
	pubDate: string;
	category: string | string[];
	guid: Guid;
	description: string;
	'wfw:commentRss': string;
	'slash:comments': number;
	'media:content': MediaContent;
}

interface Guid {
	'#text': string;
	'@_isPermaLink': string;
}

interface MediaContent {
	'@_url': string;
	'@_width': string;
	'@_height': string;
	'@_medium': string;
}
