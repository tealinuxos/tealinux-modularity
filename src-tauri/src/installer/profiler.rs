use duct::cmd;

enum Profile {
	Programmer,
	DevOps,
	Cysec,
}

impl Profile {
	fn packages(&self) -> (Vec<&'static str>, Vec<&'static str>) {
		match self {
			Self::Programmer => (
				vec!["nano", "vim", "git"],
				vec!["visual-studio-code-bin"],
			),
			Self::DevOps => (
				vec!["docker", "ansible", "terraform"],
				vec!["helm-bin"],
			),
			Self::Cysec => (
				vec!["nmap", "wireshark", "metasploit", "john"],
				vec!["burpsuite"],
			),
		}
	}

	fn install_packages(&self) {
		let (pacman_packages, yay_packages) = self.packages();

		for package in pacman_packages {
			println!("Installing pacman package: {}", package);
			let result = cmd("sudo", &["pacman", "-S", "--noconfirm", package]).run();
			if let Err(e) = result {
				eprintln!("Failed to install {}: {}", package, e);
			}
		}

		for package in yay_packages {
			println!("Installing yay package: {}", package);
			let result = cmd("yay", &["-S", "--noconfirm", package]).run();
			if let Err(e) = result {
				eprintln!("Failed to install {}: {}", package, e);
			}
		}
	}

	fn from_str(s: &str) -> Option<Self> {
		match s.to_lowercase().as_str() {
			"programmer" => Some(Self::Programmer),
			"devops" => Some(Self::DevOps),
			"cysec" => Some(Self::Cysec),
			_ => None,
		}
	}
}
