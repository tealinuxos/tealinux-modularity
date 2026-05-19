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
    devicons_str = content.split("export const validDevicons")[1].split("]")[0]
    devicons = set(re.findall(r"['\"]([^'\"]+)['\"]", devicons_str))
    
    mappings_block = re.search(r"mappings: Record<string, string> = \{(.*?)\};", content, re.DOTALL).group(1)
    mapping_keys = set(re.findall(r"['\"]([^'\"]+)['\"]\s*:", mappings_block))

missing = []
for p in packages:
    if p.lower() in devicons or p.lower() in mapping_keys:
        continue
    
    norm = p.lower().replace("-", "").replace("_", "").replace("bin", "")
    if norm in devicons:
        continue
        
    missing.append(p)

print("\n".join(sorted(missing)))
