<script lang="ts">
	import Hero from '$lib/components/home/Hero.svelte';
	import ProfileCard from '$lib/components/home/ProfileCard.svelte';
	import { invoke } from '@tauri-apps/api/core';

	const profiles = [
		{
			id: 'hacking',
			title: 'Profile Hacking',
			description: 'Advancing your hacking environment with us',
			packages: ['nmap', 'wireshark', 'metasploit']
		},
		{
			id: 'programmer',
			title: 'Profile Programmer',
			description: 'Get your tools for your development',
			packages: ['code', 'git', 'vim']
		},
		{
			id: 'backend',
			title: 'Profile BackEnd Programmer',
			description: "Get Your API's and backend development without hassle",
			packages: ['docker', 'postgresql', 'postman-bin']
		},
		{
			id: 'devops',
			title: 'Profile DevOps',
			description: 'Install all of DevOps tools, made for you',
			// Hardcoded list for demo purposes since backend profile parsing isn't fully ready
			packages: ['docker', 'ansible', 'terraform', 'kubectl', 'code']
		}
	];

	async function installProfile(profile: (typeof profiles)[0]) {
		try {
			// For the demo, we use install_packages directly with the hardcoded list
			// In production, this would call `install_profile(profile.id)`
			console.log(`Installing profile: ${profile.title}`);

			// Call the Tauri backend command we implemented earlier
			const result = await invoke('install_packages', { packages: profile.packages });
			console.log('Install result:', result);

			if (result.success) {
				alert(`Successfully installed ${profile.title}!`);
			} else {
				alert(`Failed to install: ${result.stderr}`);
			}
		} catch (e) {
			console.error('Error invoking Tauri command:', e);
			alert('Error: ' + e);
		}
	}
</script>

<div class="flex flex-col gap-6 p-6 h-full overflow-y-auto">
	<!-- Hero Section -->
	<Hero />

	<!-- Profiles Grid -->
	<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
		{#each profiles as profile}
			<ProfileCard
				title={profile.title}
				description={profile.description}
				onInstall={() => installProfile(profile)}
			/>
		{/each}
	</div>
</div>
