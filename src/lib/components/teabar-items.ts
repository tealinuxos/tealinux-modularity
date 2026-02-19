import { House, Info, SlidersHorizontal, Wrench } from '@lucide/svelte';

export const teabarItems = [
	{
		label: 'Home',
		href: '/',
		icon: House,
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
