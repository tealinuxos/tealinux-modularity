import type { CpuProfile, DnsProvider } from '$lib/types/settings';

const dns = new Set<DnsProvider>(['cloudflare', 'google', 'quad9']);

export function isDnsProvider(s: string): s is DnsProvider {
	return dns.has(s.trim().toLowerCase() as DnsProvider);
}

const cpu = new Set<CpuProfile>(['powersave', 'performance', 'ondemand']);

export function isCpuProfile(s: string): s is CpuProfile {
	return cpu.has(s.trim().toLowerCase() as CpuProfile);
}

export function sanitizeCountryChoice(list: readonly string[], pick: string): string | null {
	const t = pick.trim();
	const hit = list.find((c) => c.toLowerCase() === t.toLowerCase());
	return hit ?? null;
}
