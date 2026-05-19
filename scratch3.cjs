const si = require('simple-icons');
const fs = require('fs');

const mappings = {
    'nodejs': 'nodejs',
    'postgres': 'postgresql',
    'golang': 'go',
    'rustc': 'rust',
    'cargo': 'rust',
    'python3': 'python',
    'ruby': 'ruby',
    'visual-studio-code': 'visualstudiocode',
    'vscode': 'visualstudiocode',
    'docker-compose': 'docker',
    'mariadb-server': 'mariadb',
    'mysql-server': 'mysql',
    'beekeeper-studio-bin': 'mysql',
    'linux-firmware': 'linux',
    'android-tools': 'android',
    'jupyterlab': 'jupyter',
    'virtualbox': 'virtualbox',
    'onlyoffice-bin': 'linux',
    'clang': 'c',
    'mariadb-clients': 'mariadb',
    'k9s': 'kubernetes',
    'aws-cli': 'amazonwebservices',
    'bun-bin': 'bun',
    'rust-analyzer': 'rust',
    'insomnia-bin': 'insomnia',
    'base-devel': 'linux',
    'brave-bin': 'brave',
    'android-studio': 'androidstudio',
    'obs-studio': 'obsstudio',
    'gopls': 'go',
    'kubectl': 'kubernetes',
    'azure-cli': 'azure',
    'jdk-openjdk': 'openjdk',
    'arduino-ide-bin': 'arduino',
    'code': 'visualstudiocode',
    'jupyter-notebook': 'jupyter',
    'telegram-desktop': 'telegram',
    'python-matplotlib': 'pandas', // fallback
    'libreoffice-fresh': 'libreoffice',
    'python-pip': 'python',
    'arduino-cli': 'arduino',
    'postman-bin': 'postman',
    'lazydocker-bin': 'docker',
    'phpmyadmin': 'php',
    'google-cloud-cli': 'googlecloud',
    'python-tensorflow': 'tensorflow',
    'php-fpm': 'php',
    'php-sqlite': 'php',
    'virtualbox-host-modules-arch': 'virtualbox',
    'intellij-idea-community-edition': 'intellijidea',
    'minikube': 'kubernetes',
    'php-apache': 'php',
    'buildah': 'podman',
    'visual-studio-code-bin': 'visualstudiocode',
    'oh-my-zsh-git': 'ohmyzsh',
    'chromium': 'googlechrome',
    'python-numpy': 'numpy',
    'discord': 'discord',
    'python-pandas': 'pandas',
    'python-pytorch': 'pytorch',
    'vlc': 'vlcmediaplayer',
    'rustup': 'rust',
    'php-gd': 'php',
    'python-scipy': 'scipy',
    'openssh': 'ssh',
    'ipython': 'python',
    'make': 'c',
    'gdb': 'c',
    'valgrind': 'c',
    'xdebug': 'php',
    'gimp': 'gimp',
    'inkscape': 'inkscape',
    'blender': 'blender',
    'kdenlive': 'kdenlive',
    'audacity': 'audacity',
    'steam': 'steam',
    'lutris': 'lutris',
    'wine': 'wine',
    'winetricks': 'wine',
    'dbeaver': 'dbeaver',
    'wireshark-qt': 'wireshark',
    'github-cli': 'github',
    'gitlab': 'gitlab',
    'terraform': 'terraform',
    'ansible': 'ansible',
    'helm': 'helm',
    'podman': 'podman',
    'nginx': 'nginx',
    'apache': 'apache',
    'redis': 'redis',
    'sqlite': 'sqlite',
    'dart': 'dart',
    'flutter': 'flutter',
    'cpp_dev': 'cplusplus',
    'gcc': 'c',
    'cmake': 'cmake',
    'neovim': 'neovim',
    'vim': 'vim',
    'nano': 'nano',
    'tmux': 'tmux',
    'zsh': 'zsh',
    'git': 'git',
    'heroic-games-launcher-bin': 'epicgames',
    'mangohud': 'linux',
    'gamemode': 'linux'
};

const result = {};

for (const [pkg, siSlug] of Object.entries(mappings)) {
    // simple-icons keys are prefixed with 'si' and camelCased
    const key = "si" + siSlug.charAt(0).toUpperCase() + siSlug.slice(1).toLowerCase();
    
    // Find icon
    const icon = si[key] || Object.values(si).find(i => i.slug === siSlug || i.title.toLowerCase().replace(/ /g, '') === siSlug);
    
    if (icon) {
        result[pkg] = {
            path: icon.path,
            hex: icon.hex,
            title: icon.title
        };
    } else {
        console.log("NOT FOUND: " + siSlug + " for " + pkg);
    }
}

// Generate TS file
const tsContent = `// Automatically generated from simple-icons
export interface SimpleIconData {
    path: string;
    hex: string;
    title: string;
}

export const simpleIconsData: Record<string, SimpleIconData> = ${JSON.stringify(result, null, 4)};
`;

fs.writeFileSync('src/lib/utils/simpleicons-data.ts', tsContent);
console.log("Written to src/lib/utils/simpleicons-data.ts");
