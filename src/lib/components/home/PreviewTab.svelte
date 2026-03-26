<script lang="ts">
	import type { ProfileInfo } from '$lib/commands';
	import PackageBadge from './PackageBadge.svelte';
	import ServiceGuideCard from './ServiceGuideCard.svelte';
	import { Info } from '@lucide/svelte';

	interface Props {
		profile: ProfileInfo;
		allInstalled: boolean;
		activeTab: 'preview' | 'package';
		onTabChange: (tab: 'preview' | 'package') => void;
	}

	let { profile, allInstalled, activeTab, onTabChange }: Props = $props();

	// ── Selected package state ────────────────────────────────────────────
	let selectedPackage = $state<string | null>(null);

	function handlePackageClick(pkg: string) {
		selectedPackage = selectedPackage === pkg ? null : pkg;
	}

	// ── Package verification guides ──────────────────────────────────────
	type ServiceCommand = {
		label: string;
		cmd: string;
		type: 'setup' | 'maintenance';
	};

	type ServiceGuide = {
		description: string;
		commands: ServiceCommand[];
	};

	const packageGuides: Record<string, ServiceGuide> = {
		// ── Services ──────────────────────────────────────────────────
		docker: {
			description: 'Container runtime. Verify the daemon is running and list active containers.',
			commands: [
				{ label: 'Check Docker version', cmd: 'docker version', type: 'maintenance' },
				{ label: 'List running containers', cmd: 'docker ps', type: 'maintenance' },
				{ label: 'List all containers', cmd: 'docker ps -a', type: 'maintenance' },
				{
					label: 'Add user to docker group',
					cmd: 'sudo usermod -aG docker $USER && newgrp docker',
					type: 'setup'
				}
			]
		},
		postgresql: {
			description:
				'Relational database. Requires initdb on first boot before the service can start.',
			commands: [
				{ label: 'Check PostgreSQL version', cmd: 'psql --version', type: 'maintenance' },
				{
					label: 'Check service status',
					cmd: 'systemctl status postgresql',
					type: 'maintenance'
				},
				{
					label: 'Initialize database (first time only)',
					cmd: 'sudo -u postgres initdb -D /var/lib/postgres/data',
					type: 'setup'
				},
				{ label: 'Connect to PostgreSQL', cmd: 'sudo -u postgres psql', type: 'maintenance' }
			]
		},
		redis: {
			description: 'In-memory key-value store for caching and sessions.',
			commands: [
				{ label: 'Check Redis version', cmd: 'redis-cli --version', type: 'maintenance' },
				{ label: 'Check service status', cmd: 'systemctl status redis', type: 'maintenance' },
				{ label: 'Ping Redis daemon', cmd: 'redis-cli ping', type: 'maintenance' },
				{ label: 'Monitor live traffic', cmd: 'redis-cli monitor', type: 'maintenance' }
			]
		},
		nginx: {
			description: 'High-performance web server and reverse proxy.',
			commands: [
				{ label: 'Check nginx version', cmd: 'nginx -v', type: 'maintenance' },
				{ label: 'Validate configuration', cmd: 'sudo nginx -t', type: 'setup' },
				{
					label: 'Check service status',
					cmd: 'systemctl status nginx',
					type: 'maintenance'
				},
				{ label: 'Test HTTP response', cmd: 'curl -I http://localhost', type: 'maintenance' }
			]
		},

		// ── Dev Tools ─────────────────────────────────────────────────
		git: {
			description: 'Distributed version control system. Essential for source code management.',
			commands: [
				{ label: 'Check Git version', cmd: 'git --version', type: 'maintenance' },
				{ label: 'Set your name', cmd: 'git config --global user.name "Your Name"', type: 'setup' },
				{
					label: 'Set your email',
					cmd: 'git config --global user.email "you@example.com"',
					type: 'setup'
				},
				{ label: 'View config', cmd: 'git config --list', type: 'maintenance' }
			]
		},
		vim: {
			description: 'Classic modal text editor with powerful keybindings.',
			commands: [
				{ label: 'Check Vim version', cmd: 'vim --version | head -1', type: 'maintenance' },
				{ label: 'Open Vim tutor', cmd: 'vimtutor', type: 'setup' },
				{ label: 'Edit config', cmd: 'vim ~/.vimrc', type: 'setup' }
			]
		},
		neovim: {
			description: 'Modern fork of Vim with Lua scripting and LSP support built-in.',
			commands: [
				{ label: 'Check Neovim version', cmd: 'nvim --version', type: 'maintenance' },
				{ label: 'Open health check', cmd: 'nvim +checkhealth', type: 'maintenance' },
				{ label: 'Edit config', cmd: 'nvim ~/.config/nvim/init.lua', type: 'setup' }
			]
		},
		cmake: {
			description: 'Cross-platform build system generator for C/C++ projects.',
			commands: [
				{ label: 'Check CMake version', cmd: 'cmake --version', type: 'maintenance' },
				{
					label: 'Generate build files',
					cmd: 'cmake -B build -S .',
					type: 'setup'
				},
				{ label: 'Build project', cmd: 'cmake --build build', type: 'maintenance' }
			]
		},
		'base-devel': {
			description:
				'Meta-package providing essential build tools (gcc, make, binutils, etc.) needed to compile software from source.',
			commands: [
				{ label: 'Check GCC version', cmd: 'gcc --version', type: 'maintenance' },
				{ label: 'Check Make version', cmd: 'make --version', type: 'maintenance' },
				{
					label: 'List installed group packages',
					cmd: 'pacman -Qg base-devel',
					type: 'maintenance'
				}
			]
		},

		// ── Runtimes & Languages ──────────────────────────────────────
		nodejs: {
			description: 'JavaScript runtime built on V8 engine for server-side development.',
			commands: [
				{ label: 'Check Node.js version', cmd: 'node --version', type: 'maintenance' },
				{ label: 'Check npm version', cmd: 'npm --version', type: 'maintenance' },
				{ label: 'Init new project', cmd: 'npm init -y', type: 'setup' },
				{ label: 'List global packages', cmd: 'npm list -g --depth=0', type: 'maintenance' }
			]
		},
		npm: {
			description: 'Node.js package manager for installing and managing JavaScript packages.',
			commands: [
				{ label: 'Check npm version', cmd: 'npm --version', type: 'maintenance' },
				{ label: 'Init new project', cmd: 'npm init -y', type: 'setup' },
				{ label: 'List global packages', cmd: 'npm list -g --depth=0', type: 'maintenance' },
				{ label: 'Update npm', cmd: 'npm install -g npm@latest', type: 'setup' }
			]
		},
		python: {
			description: 'Versatile programming language for scripting, web dev, data science, and AI.',
			commands: [
				{ label: 'Check Python version', cmd: 'python --version', type: 'maintenance' },
				{ label: 'Check pip version', cmd: 'pip --version', type: 'maintenance' },
				{
					label: 'Create virtual environment',
					cmd: 'python -m venv .venv && source .venv/bin/activate',
					type: 'setup'
				},
				{ label: 'List installed packages', cmd: 'pip list', type: 'maintenance' }
			]
		},
		go: {
			description: 'Statically typed language by Google, ideal for backend services and CLI tools.',
			commands: [
				{ label: 'Check Go version', cmd: 'go version', type: 'maintenance' },
				{ label: 'Init new module', cmd: 'go mod init myproject', type: 'setup' },
				{ label: 'Check environment', cmd: 'go env', type: 'maintenance' },
				{ label: 'Run tests', cmd: 'go test ./...', type: 'maintenance' }
			]
		},

		// ── DevOps Tools ──────────────────────────────────────────────
		'docker-compose': {
			description: 'Define and run multi-container Docker applications with YAML configuration.',
			commands: [
				{ label: 'Check version', cmd: 'docker compose version', type: 'maintenance' },
				{ label: 'Start services', cmd: 'docker compose up -d', type: 'maintenance' },
				{ label: 'Stop services', cmd: 'docker compose down', type: 'maintenance' },
				{ label: 'View running services', cmd: 'docker compose ps', type: 'maintenance' }
			]
		},
		ansible: {
			description: 'Agentless automation tool for configuration management and app deployment.',
			commands: [
				{ label: 'Check Ansible version', cmd: 'ansible --version', type: 'maintenance' },
				{
					label: 'Ping localhost',
					cmd: 'ansible localhost -m ping',
					type: 'maintenance'
				},
				{
					label: 'Run a playbook',
					cmd: 'ansible-playbook playbook.yml',
					type: 'setup'
				}
			]
		},
		terraform: {
			description: 'Infrastructure as Code tool for provisioning cloud resources declaratively.',
			commands: [
				{ label: 'Check Terraform version', cmd: 'terraform version', type: 'maintenance' },
				{ label: 'Initialize project', cmd: 'terraform init', type: 'setup' },
				{ label: 'Plan changes', cmd: 'terraform plan', type: 'maintenance' },
				{ label: 'Apply changes', cmd: 'terraform apply', type: 'setup' }
			]
		},
		kubectl: {
			description: 'CLI for Kubernetes cluster management and resource deployment.',
			commands: [
				{ label: 'Check kubectl version', cmd: 'kubectl version --client', type: 'maintenance' },
				{ label: 'Get cluster info', cmd: 'kubectl cluster-info', type: 'maintenance' },
				{ label: 'List pods', cmd: 'kubectl get pods -A', type: 'maintenance' },
				{ label: 'List services', cmd: 'kubectl get svc -A', type: 'maintenance' }
			]
		},
		minikube: {
			description: 'Run a single-node Kubernetes cluster locally for development and testing.',
			commands: [
				{ label: 'Check Minikube version', cmd: 'minikube version', type: 'maintenance' },
				{ label: 'Start cluster', cmd: 'minikube start', type: 'setup' },
				{ label: 'Check status', cmd: 'minikube status', type: 'maintenance' },
				{ label: 'Open dashboard', cmd: 'minikube dashboard', type: 'maintenance' }
			]
		},
		helm: {
			description: 'Package manager for Kubernetes — deploy apps with reusable charts.',
			commands: [
				{ label: 'Check Helm version', cmd: 'helm version', type: 'maintenance' },
				{
					label: 'Add stable repo',
					cmd: 'helm repo add stable https://charts.helm.sh/stable',
					type: 'setup'
				},
				{ label: 'Update repos', cmd: 'helm repo update', type: 'maintenance' },
				{ label: 'List releases', cmd: 'helm list -A', type: 'maintenance' }
			]
		},
		code: {
			description: 'Visual Studio Code — popular extensible code editor by Microsoft.',
			commands: [
				{ label: 'Check VS Code version', cmd: 'code --version', type: 'maintenance' },
				{ label: 'List extensions', cmd: 'code --list-extensions', type: 'maintenance' },
				{
					label: 'Install an extension',
					cmd: 'code --install-extension ms-python.python',
					type: 'setup'
				}
			]
		},
		htop: {
			description: 'Interactive process viewer — a modern alternative to top.',
			commands: [
				{ label: 'Check htop version', cmd: 'htop --version', type: 'maintenance' },
				{ label: 'Launch htop', cmd: 'htop', type: 'maintenance' }
			]
		},

		// ── Security & Hacking Tools ──────────────────────────────────
		nmap: {
			description: 'Network exploration and security auditing tool for port scanning.',
			commands: [
				{ label: 'Check Nmap version', cmd: 'nmap --version', type: 'maintenance' },
				{ label: 'Scan localhost', cmd: 'nmap -sV localhost', type: 'maintenance' },
				{ label: 'Quick network scan', cmd: 'nmap -sn 192.168.1.0/24', type: 'maintenance' }
			]
		},
		'wireshark-qt': {
			description: 'Network protocol analyzer with GUI for deep packet inspection.',
			commands: [
				{ label: 'Check version', cmd: 'wireshark --version | head -1', type: 'maintenance' },
				{
					label: 'Add user to wireshark group',
					cmd: 'sudo usermod -aG wireshark $USER',
					type: 'setup'
				},
				{ label: 'CLI capture', cmd: 'tshark -i eth0 -c 10', type: 'maintenance' }
			]
		},
		john: {
			description: 'Password cracker supporting many cipher and hash types.',
			commands: [
				{ label: 'Check John version', cmd: 'john --help | head -5', type: 'maintenance' },
				{
					label: 'Benchmark performance',
					cmd: 'john --test',
					type: 'maintenance'
				}
			]
		},
		'aircrack-ng': {
			description:
				'WiFi security auditing toolkit for monitoring, attacking, and testing networks.',
			commands: [
				{ label: 'Check version', cmd: 'aircrack-ng --help | head -3', type: 'maintenance' },
				{
					label: 'List wireless interfaces',
					cmd: 'airmon-ng',
					type: 'maintenance'
				}
			]
		},
		hashcat: {
			description: 'Advanced GPU-accelerated password recovery utility.',
			commands: [
				{ label: 'Check Hashcat version', cmd: 'hashcat --version', type: 'maintenance' },
				{ label: 'Benchmark all hashes', cmd: 'hashcat -b', type: 'maintenance' },
				{
					label: 'List hash types',
					cmd: 'hashcat --help | grep -i "hash-type"',
					type: 'maintenance'
				}
			]
		},
		whois: {
			description: 'Query tool for domain registration and IP ownership information.',
			commands: [
				{ label: 'Lookup a domain', cmd: 'whois example.com', type: 'maintenance' },
				{ label: 'Lookup an IP', cmd: 'whois 8.8.8.8', type: 'maintenance' }
			]
		},
		tcpdump: {
			description: 'Command-line packet capture and analysis tool.',
			commands: [
				{ label: 'Check tcpdump version', cmd: 'tcpdump --version', type: 'maintenance' },
				{
					label: 'Capture 10 packets',
					cmd: 'sudo tcpdump -c 10 -i any',
					type: 'maintenance'
				},
				{
					label: 'Capture to file',
					cmd: 'sudo tcpdump -c 100 -w capture.pcap',
					type: 'maintenance'
				}
			]
		},
		netcat: {
			description: 'Versatile networking tool for reading/writing across TCP/UDP connections.',
			commands: [
				{ label: 'Check netcat help', cmd: 'nc -h 2>&1 | head -3', type: 'maintenance' },
				{ label: 'Port scan', cmd: 'nc -zv localhost 1-1024', type: 'maintenance' },
				{ label: 'Listen on port', cmd: 'nc -l -p 4444', type: 'setup' }
			]
		},
		sqlmap: {
			description: 'Automatic SQL injection detection and exploitation tool.',
			commands: [
				{ label: 'Check sqlmap version', cmd: 'sqlmap --version', type: 'maintenance' },
				{
					label: 'Basic scan example',
					cmd: 'sqlmap -u "http://target.com/page?id=1" --batch',
					type: 'maintenance'
				}
			]
		},

		// ── AUR Packages ──────────────────────────────────────────────
		'visual-studio-code-bin': {
			description: 'Visual Studio Code — popular extensible code editor (AUR binary release).',
			commands: [
				{ label: 'Check VS Code version', cmd: 'code --version', type: 'maintenance' },
				{ label: 'List extensions', cmd: 'code --list-extensions', type: 'maintenance' },
				{
					label: 'Install Python extension',
					cmd: 'code --install-extension ms-python.python',
					type: 'setup'
				}
			]
		},
		'postman-bin': {
			description: 'API development environment for building, testing, and documenting APIs.',
			commands: [
				{ label: 'Launch Postman', cmd: 'postman', type: 'maintenance' },
				{ label: 'Check installation', cmd: 'which postman', type: 'maintenance' }
			]
		},
		burpsuite: {
			description:
				'Integrated platform for web application security testing and vulnerability scanning.',
			commands: [
				{ label: 'Launch BurpSuite', cmd: 'burpsuite', type: 'maintenance' },
				{ label: 'Check installation', cmd: 'which burpsuite', type: 'maintenance' }
			]
		}
	};

	// ── Derived: guide for selected package ──────────────────────────────
	let selectedGuide = $derived(selectedPackage ? (packageGuides[selectedPackage] ?? null) : null);
	let isService = $derived(
		selectedPackage ? profile.services_enable.includes(selectedPackage) : false
	);

	// ── Copy-to-clipboard state ──────────────────────────────────────────
	let copiedCmd = $state<string | null>(null);
	let copyTimeout: ReturnType<typeof setTimeout> | null = null;

	async function copyToClipboard(cmd: string) {
		try {
			await navigator.clipboard.writeText(cmd);
		} catch {
			// Fallback for environments without clipboard API
			const ta = document.createElement('textarea');
			ta.value = cmd;
			ta.style.position = 'fixed';
			ta.style.opacity = '0';
			document.body.appendChild(ta);
			ta.select();
			document.execCommand('copy');
			document.body.removeChild(ta);
		}
		copiedCmd = cmd;
		if (copyTimeout) clearTimeout(copyTimeout);
		copyTimeout = setTimeout(() => {
			copiedCmd = null;
		}, 1500);
	}
</script>

<div class="flex flex-col">
	<!-- ── Shared Header ──────────────────────────── -->
	<div
		class="flex items-start justify-between gap-6 px-8 pt-7 pb-5 max-sm:flex-col max-sm:px-5 max-sm:pt-5 max-sm:pb-4"
	>
		<div class="flex flex-col gap-1">
			<h1 class="m-0 text-2xl font-extrabold leading-tight tracking-tight text-foreground">
				{profile.name}
			</h1>
			<p class="m-0 max-w-[22rem] text-[0.8rem] leading-relaxed text-muted-foreground">
				{profile.description}
			</p>
		</div>
		<div class="flex shrink-0 items-center rounded-lg border border-border bg-muted/40 p-0.5">
			<button
				onclick={() => onTabChange('preview')}
				class={`cursor-pointer rounded-md px-4 py-1.5 text-sm font-medium transition-all ${activeTab === 'preview' ? 'bg-background text-[#54CD4C] shadow-sm' : 'text-muted-foreground hover:text-foreground'}`}
			>
				Preview
			</button>
			<button
				onclick={() => onTabChange('package')}
				class={`cursor-pointer rounded-md px-4 py-1.5 text-sm font-medium transition-all ${activeTab === 'package' ? 'bg-background text-[#54CD4C] shadow-sm' : 'text-muted-foreground hover:text-foreground'}`}
			>
				Package
			</button>
		</div>
	</div>

	<hr class="mx-8 border-t border-border/50 max-sm:mx-5" />

	<!-- ── Preview Content ───────────────────────── -->
	{#if activeTab === 'preview'}
		<div class="flex flex-col gap-5 px-8 pt-6 pb-8 max-sm:p-5">
			<div class="space-y-6 rounded-2xl border border-border bg-card p-6">
				<div class="flex items-center justify-between">
					<h2 class="text-lg font-semibold">Installation Preview</h2>
					{#if selectedPackage}
						<button
							onclick={() => (selectedPackage = null)}
							class="cursor-pointer rounded-md border border-border bg-muted/50 px-3 py-1 text-xs text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
						>
							Clear Selection
						</button>
					{/if}
				</div>

				<div class="space-y-6">
					<!-- Official Packages -->
					{#if profile.packages_install.length > 0}
						<div class="space-y-3">
							<h3 class="text-sm font-medium uppercase tracking-wider text-muted-foreground">
								Official Packages
								<span class="ml-1 text-xs normal-case tracking-normal opacity-60">
									— click to see guide
								</span>
							</h3>
							<div class="flex flex-wrap gap-2">
								{#each profile.packages_install as pkg}
									<PackageBadge
										name={pkg}
										variant="official"
										isSelected={selectedPackage === pkg}
										onclick={() => handlePackageClick(pkg)}
									/>
								{/each}
							</div>
						</div>
					{/if}

					<!-- AUR Packages -->
					{#if profile.packages_aur.length > 0}
						<div class="space-y-3">
							<h3 class="text-sm font-medium uppercase tracking-wider text-muted-foreground">
								AUR Packages
								<span class="ml-1 text-xs normal-case tracking-normal opacity-60">
									— click to see guide
								</span>
							</h3>
							<div class="flex flex-wrap gap-2">
								{#each profile.packages_aur as pkg}
									<PackageBadge
										name={pkg}
										variant="aur"
										isSelected={selectedPackage === pkg}
										onclick={() => handlePackageClick(pkg)}
									/>
								{/each}
							</div>
						</div>
					{/if}

					<!-- ── Service Configuration Guide ─────────────── -->
					<div class="space-y-3">
						<h3 class="text-sm font-medium uppercase tracking-wider text-muted-foreground">
							Services Configuration & Guides
						</h3>

						{#if selectedPackage && selectedGuide}
							<!-- Show guide for selected package -->
							<div class="animate-in fade-in slide-in-from-bottom-2 duration-200">
								{#if isService}
									<div
										class="mb-3 flex items-center gap-2 rounded-lg border border-[#54CD4C]/20 bg-[#54CD4C]/5 px-3 py-2"
									>
										<div class="h-2 w-2 shrink-0 rounded-full bg-[#54CD4C] animate-pulse"></div>
										<span class="text-xs font-medium text-[#54CD4C]">
											This package is a service — it will be enabled automatically on install
										</span>
									</div>
								{/if}
								<ServiceGuideCard
									svc={selectedPackage}
									guide={selectedGuide}
									{copiedCmd}
									onCopy={copyToClipboard}
								/>
							</div>
						{:else if selectedPackage && !selectedGuide}
							<!-- Package selected but no guide available -->
							<div
								class="flex flex-col items-center gap-3 rounded-xl border border-border bg-muted/30 py-8"
							>
								<div class="flex h-10 w-10 items-center justify-center rounded-full bg-muted">
									<Info class="h-5 w-5 text-muted-foreground" />
								</div>
								<div class="text-center">
									<p class="m-0 text-sm font-semibold text-foreground capitalize">
										{selectedPackage}
									</p>
									<p class="m-0 mt-1 text-xs text-muted-foreground">
										No specific guide available for this package yet.
									</p>
								</div>
							</div>
						{:else}
							<!-- No package selected — show instruction -->
							<div
								class="flex flex-col items-center gap-3 rounded-xl border border-dashed border-border/60 bg-muted/20 py-8"
							>
								<div class="flex h-10 w-10 items-center justify-center rounded-full bg-muted/60">
									<Info class="h-5 w-5 text-muted-foreground/60" />
								</div>
								<p class="m-0 text-center text-sm text-muted-foreground">
									Click on a package above to view its setup guide and commands
								</p>
							</div>
						{/if}
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>
