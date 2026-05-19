import { QueryClient } from '@tanstack/svelte-query';

const ONE_HOUR = 1000 * 60 * 60;

export const queryClient = new QueryClient({
	defaultOptions: {
		queries: {
			staleTime: ONE_HOUR,
			gcTime: ONE_HOUR,
			refetchInterval: ONE_HOUR,
			experimental_prefetchInRender: true
		}
	}
});
