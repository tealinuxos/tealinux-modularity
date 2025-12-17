import { House, Info, SlidersHorizontal, Wrench } from '@lucide/svelte';

export const teabarItems = [
	{
		href: '/',
		label: 'Home',
		icon: House
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
