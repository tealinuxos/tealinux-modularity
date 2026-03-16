import { House, Info, Package, SlidersHorizontal, Wrench } from '@lucide/svelte';

export const teabarItems = [
	{
		label: 'Home',
		href: '/home',
		icon: House,
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
