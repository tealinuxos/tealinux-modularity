import { 
	Shield, 
	CodeXml, 
	Server, 
	Wrench, 
	Package, 
	Globe, 
	Gamepad2, 
	Cpu, 
	GraduationCap, 
	Clapperboard, 
	Briefcase,
	type Icon as IconType 
} from '@lucide/svelte';

export const categoryIcons: Record<string, typeof IconType> = {
	security: Shield,
	development: CodeXml,
	server: Server,
	tools: Wrench,
	office: Briefcase,
	multimedia: Clapperboard,
	internet: Globe,
	games: Gamepad2,
	system: Cpu,
	education: GraduationCap,
	default: Package
};

export const categoryColors: Record<string, string> = {
	security: 'bg-red-500/10 ring-1 ring-red-500/15',
	development: 'bg-blue-500/10 ring-1 ring-blue-500/15',
	server: 'bg-purple-500/10 ring-1 ring-purple-500/15',
	tools: 'bg-amber-500/10 ring-1 ring-amber-500/15',
	office: 'bg-emerald-500/10 ring-1 ring-emerald-500/15',
	multimedia: 'bg-pink-500/10 ring-1 ring-pink-500/15',
	internet: 'bg-cyan-500/10 ring-1 ring-cyan-500/15',
	games: 'bg-violet-500/10 ring-1 ring-violet-500/15',
	system: 'bg-slate-500/10 ring-1 ring-slate-500/15',
	education: 'bg-orange-500/10 ring-1 ring-orange-500/15',
	default: 'bg-[color:var(--accent-green)]/10 ring-1 ring-[color:var(--accent-green)]/15'
};

export const categoryBadgeColors: Record<string, string> = {
	security: 'bg-red-500/10 text-red-300 border-red-500/20',
	development: 'bg-blue-500/10 text-blue-300 border-blue-500/20',
	server: 'bg-purple-500/10 text-purple-300 border-purple-500/20',
	tools: 'bg-amber-500/10 text-amber-300 border-amber-500/20',
	office: 'bg-emerald-500/10 text-emerald-300 border-emerald-500/20',
	multimedia: 'bg-pink-500/10 text-pink-300 border-pink-500/20',
	internet: 'bg-cyan-500/10 text-cyan-300 border-cyan-500/20',
	games: 'bg-violet-500/10 text-violet-300 border-violet-500/20',
	system: 'bg-slate-500/10 text-slate-300 border-slate-500/20',
	education: 'bg-orange-500/10 text-orange-300 border-orange-500/20',
	default:
		'bg-[color:var(--accent-green)]/10 text-[color:var(--accent-green)] border-[color:var(--accent-green)]/20'
};

export function getCategoryIcon(category: string) {
	return categoryIcons[category] || categoryIcons['default'];
}

export function getCategoryColor(category: string) {
	return categoryColors[category] || categoryColors['default'];
}

export function getCategoryBadgeColor(category: string) {
	return categoryBadgeColors[category] || categoryBadgeColors['default'];
}
