import glob, re

profiles = glob.glob("profiles/*.toml")
packages = set()
for profile in profiles:
    with open(profile) as f:
        content = f.read()
        for block in re.findall(r"(?:install|aur)\s*=\s*\[(.*?)\]", content, re.DOTALL):
            pkgs = re.findall(r"\"([^\"]+)\"", block)
            packages.update(pkgs)

with open("src/lib/utils/devicon.ts") as f:
    content = f.read()
    devicons = set(re.findall(r"\s+['\"]([^'\"]+)['\"]", content.split("export const validDevicons")[1].split("]")[0]))

mappings = {}
for p in packages:
    if p in devicons:
        continue
    
    known = {
        "visual-studio-code-bin": "vscode",
        "code": "vscode",
        "android-studio": "androidstudio",
        "nodejs": "nodejs",
        "npm": "npm",
        "yarn": "yarn",
        "pnpm": "pnpm",
        "bun-bin": "bun",
        "docker-compose": "docker",
        "mariadb-clients": "mariadb",
        "phpmyadmin": "php",
        "php-apache": "php",
        "php-fpm": "php",
        "php-gd": "php",
        "php-sqlite": "php",
        "python-matplotlib": "matplotlib",
        "python-numpy": "numpy",
        "python-pandas": "pandas",
        "python-pytorch": "pytorch",
        "python-tensorflow": "tensorflow",
        "python-pip": "python",
        "jupyter-notebook": "jupyter",
        "jupyterlab": "jupyter",
        "rustup": "rust",
        "rust-analyzer": "rust",
        "cargo": "rust",
        "jdk-openjdk": "java",
        "intellij-idea-community-edition": "intellij",
        "google-cloud-cli": "googlecloud",
        "aws-cli": "amazonwebservices",
        "azure-cli": "azure",
        "brave-bin": "chrome",
        "chromium": "chrome",
        "discord": "discordjs",
        "gimp": "gimp",
        "inkscape": "inkscape",
        "blender": "blender",
        "obs-studio": "obsstudio",
        "virtualbox": "virtualbox",
        "virtualbox-host-modules-arch": "virtualbox",
        "insomnia-bin": "insomnia",
        "dbeaver": "dbeaver",
        "beekeeper-studio-bin": "mysql",
        "mysql-server": "mysql",
        "postgresql": "postgresql",
        "sqlite": "sqlite",
        "redis": "redis",
        "nginx": "nginx",
        "apache": "apache",
        "go": "go",
        "gopls": "go",
        "ruby": "ruby",
        "dart": "dart",
        "flutter": "flutter",
        "cpp_dev": "cplusplus",
        "gcc": "c",
        "clang": "c",
        "cmake": "cmake",
        "arduino-cli": "arduino",
        "arduino-ide-bin": "arduino",
        "base-devel": "linux",
        "linux-firmware": "linux",
        "neovim": "neovim",
        "vim": "vim",
        "nano": "nano",
        "tmux": "tmux",
        "zsh": "zsh",
        "oh-my-zsh-git": "ohmyzsh",
        "git": "git",
        "github-cli": "github",
        "gitlab": "gitlab",
        "terraform": "terraform",
        "ansible": "ansible",
        "kubernetes": "kubernetes",
        "kubectl": "kubernetes",
        "minikube": "kubernetes",
        "helm": "helm",
        "k9s": "kubernetes",
        "podman": "podman",
        "buildah": "podman",
        "lazydocker-bin": "docker",
        "libreoffice-fresh": "linux",
        "obs-studio": "linux",
        "vlc": "linux",
        "telegram-desktop": "linux",
        "onlyoffice-bin": "linux",
    }
    
    if p in known:
        mappings[p] = known[p]
        continue

    # fallback
    norm = p.replace("-", "").replace("_", "").replace("bin", "")
    for d in devicons:
        if d == norm or p.startswith(d + "-"):
            mappings[p] = d
            break
            
print("\n".join(f"        '{k}': '{v}'," for k, v in mappings.items()))
