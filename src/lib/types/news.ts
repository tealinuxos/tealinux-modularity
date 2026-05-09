/** News item mirrored from libs `ParsedNewsItem` / Tauri `ParsedNewsItemDto`. */
export type ParsedNewsItem = {
	url: string;
	title: string;
	descriptive: string;
	thumbnail?: string | null;
};
