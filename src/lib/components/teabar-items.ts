import { House, Info, Package, Settings, SlidersHorizontal, TrendingUp } from '@lucide/svelte';

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
		href: '/aur',
		label: 'AUR Packages',
		icon: Package
	},
	{
		href: '/settings',
		label: 'Settings',
		icon: Settings
	},
	{
		href: '/grub',
		label: 'Boot Theme',
		icon: SlidersHorizontal
	}
];
