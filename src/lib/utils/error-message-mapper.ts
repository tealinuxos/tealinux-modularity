import type { LocalModulariteaError } from '$lib/commands';

export const errorMessageMapper = (error: LocalModulariteaError): string => {
	if (typeof error === 'object' && error !== null) {
		if ('type' in error) {
			switch (error.type) {
				case 'GrubError':
					return `GRUB Error: ${error.data?.reason || 'Unknown error'}`;
				case 'PrivilegeError':
					return `Privilege Error: ${error.data?.reason || 'Permission denied'}`;
				case 'PkexecNotFound':
					return 'pkexec not found - cannot apply theme without root privileges';
				case 'PolkitCancelled':
					return 'Authorization cancelled by user';
				case 'FilesystemError':
					return `Filesystem Error: ${error.data?.operation || 'Unknown operation'} failed`;
				case 'CommandError':
					return `Command Error: ${error.data?.stderr || 'Unknown error'}`;
				case 'InternalError':
					return `Internal Error: ${error.data}`;
				default:
					return `Error: ${error.type}`;
			}
		}
	}
	return 'An unknown error occurred';
};
