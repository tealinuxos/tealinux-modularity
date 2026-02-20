import { Shield, Code2, Server, Wrench, Package } from 'lucide-svelte';

export const categoryIcons: Record<string, any> = {
    security: Shield,
    development: Code2,
    server: Server,
    tools: Wrench,
    default: Package
};

export const categoryColors: Record<string, string> = {
    security: 'from-red-500/20 to-red-600/5 border-red-500/30 text-red-400',
    development: 'from-blue-500/20 to-blue-600/5 border-blue-500/30 text-blue-400',
    server: 'from-purple-500/20 to-purple-600/5 border-purple-500/30 text-purple-400',
    tools: 'from-amber-500/20 to-amber-600/5 border-amber-500/30 text-amber-400',
    default: 'from-green-500/20 to-green-600/5 border-green-500/30 text-green-400'
};

export const categoryBadgeColors: Record<string, string> = {
    security: 'bg-red-500/20 text-red-400 border-red-500/30',
    development: 'bg-blue-500/20 text-blue-400 border-blue-500/30',
    server: 'bg-purple-500/20 text-purple-400 border-purple-500/30',
    tools: 'bg-amber-500/20 text-amber-400 border-amber-500/30',
    default: 'bg-green-500/20 text-green-400 border-green-500/30'
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
