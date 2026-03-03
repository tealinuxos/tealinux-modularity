import { House, Info, SlidersHorizontal, TrendingUp, Wrench } from '@lucide/svelte';

export const teabarItems = [
	{
		label: 'Home',
		href: '/home',
		icon: House
	},
	{
		label: 'Latest News',
		href: '/news',
		icon: TrendingUp
	},
	{
		href: '/sysinfo',
		label: 'System Information',
		icon: Info
	},
	{
		href: '/tools',
		label: 'Tools',
		icon: Wrench
	},
	{
		href: '/grub',
		label: 'GRUB Changer',
		icon: SlidersHorizontal
	}
];
