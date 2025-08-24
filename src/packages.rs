use ansi_term::Colour::{Blue, Cyan, Green, Red, Yellow};
use std::collections::HashSet;
use std::io::{self, Write};
use std::process::Command;

use crate::config::{get_base_packages, get_packages, get_system_packages};

pub fn get_installed_packages() -> Result<Vec<String>, std::io::Error> {
	let output = Command::new("pacman")
		.args(["-Qq"])
		.output()
		.map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to run pacman: {}", e)))?;

	if !output.status.success() {
		return Err(io::Error::new(io::ErrorKind::Other, "pacman command failed"));
	}

	let packages = String::from_utf8_lossy(&output.stdout)
		.lines()
		.map(|line| line.trim().to_string())
		.filter(|line| !line.is_empty())
		.collect();

	Ok(packages)
}

pub fn get_explicitly_installed_packages() -> Result<Vec<String>, std::io::Error> {
	let output = Command::new("pacman")
		.args(["-Qqe"])
		.output()
		.map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to run pacman: {}", e)))?;

	if !output.status.success() {
		return Err(io::Error::new(io::ErrorKind::Other, "pacman command failed"));
	}

	let packages = String::from_utf8_lossy(&output.stdout)
		.lines()
		.map(|line| line.trim().to_string())
		.filter(|line| !line.is_empty())
		.collect();

	Ok(packages)
}

pub fn get_readonly_packages() -> HashSet<String> {
	let mut readonly = HashSet::new();
	readonly.extend(get_base_packages());
	readonly.extend(get_system_packages());
	readonly
}

pub fn get_managed_packages() -> Result<HashSet<String>, std::io::Error> {
	let packages_config = get_packages()?;
	let mut managed = HashSet::new();

	for (_, package_list) in packages_config {
		managed.extend(package_list);
	}

	Ok(managed)
}

pub fn packages_diff() -> Result<(), std::io::Error> {
	let explicitly_installed = get_explicitly_installed_packages()?;
	let explicitly_installed_set: HashSet<String> = explicitly_installed.into_iter().collect();

	let all_installed = get_installed_packages()?;
	let all_installed_set: HashSet<String> = all_installed.into_iter().collect();

	let readonly = get_readonly_packages();
	let managed = get_managed_packages()?;

	// Packages that should be installed but aren't (check against all packages)
	let missing: Vec<&String> = managed.difference(&all_installed_set).collect();

	// Packages that are explicitly installed but not in config (excluding readonly)
	let extra: Vec<&String> =
		explicitly_installed_set.difference(&managed).filter(|pkg| !readonly.contains(*pkg)).collect();

	// Readonly packages status (check against all packages)
	let readonly_missing: Vec<&String> = readonly.difference(&all_installed_set).collect();

	println!("{}", Blue.paint("📦 Package Status Overview"));
	println!();

	if !readonly_missing.is_empty() {
		println!("{} {}", Yellow.paint("⚠️  Missing system packages:"), readonly_missing.len());
		for pkg in &readonly_missing {
			println!("   {}", Yellow.paint(format!("- {}", pkg)));
		}
		println!();
	}

	if !missing.is_empty() {
		println!("{} {}", Green.paint("📥 Packages to install:"), missing.len());
		for pkg in &missing {
			println!("   {}", Green.paint(format!("+ {}", pkg)));
		}
		println!();
	}

	if !extra.is_empty() {
		println!("{} {}", Red.paint("📤 Extra packages (not in config):"), extra.len());
		for pkg in &extra {
			println!("   {}", Red.paint(format!("- {}", pkg)));
		}
		println!();
	}

	if missing.is_empty() && extra.is_empty() && readonly_missing.is_empty() {
		println!("{}", Green.paint("✅ All packages are in sync!"));
	}

	Ok(())
}

pub fn packages_sync() -> Result<(), std::io::Error> {
	let all_installed = get_installed_packages()?;
	let all_installed_set: HashSet<String> = all_installed.into_iter().collect();

	let managed = get_managed_packages()?;
	let missing: Vec<&String> = managed.difference(&all_installed_set).collect();

	if missing.is_empty() {
		println!("{}", Green.paint("✅ All packages are already installed!"));
		return Ok(());
	}

	println!("{} {}", Cyan.paint("📦 Packages to install:"), missing.len());
	for pkg in &missing {
		println!("   {}", Green.paint(format!("+ {}", pkg)));
	}
	println!();

	print!("{}", Yellow.paint("Do you want to proceed? [y/N]: "));
	io::stdout().flush()?;

	let mut input = String::new();
	io::stdin().read_line(&mut input)?;
	let input = input.trim().to_lowercase();

	if input != "y" && input != "yes" {
		println!("Cancelled.");
		return Ok(());
	}

	let packages: Vec<&str> = missing.iter().map(|s| s.as_str()).collect();
	let status = Command::new("sudo")
		.args(["pacman", "-S", "--needed"])
		.args(&packages)
		.status()
		.map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to run pacman: {}", e)))?;

	if status.success() {
		println!("{}", Green.paint("✅ Packages installed successfully!"));
	} else {
		println!("{}", Red.paint("❌ Package installation failed!"));
	}

	Ok(())
}

pub fn packages_export() -> Result<(), std::io::Error> {
	let installed = get_explicitly_installed_packages()?;
	let readonly = get_readonly_packages();

	let user_packages: Vec<String> = installed.into_iter().filter(|pkg| !readonly.contains(pkg)).collect();

	println!("{}", Blue.paint("📋 Exportable packages (excluding system packages):"));
	println!();

	for pkg in &user_packages {
		println!("{}", pkg);
	}

	println!();
	println!("{}", Yellow.paint(format!("Total: {} packages", user_packages.len())));
	println!();
	println!("{}", Cyan.paint("💡 Add these to your dotfiles.toml [packages] section"));

	Ok(())
}
