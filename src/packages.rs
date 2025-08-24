use ansi_term::Colour::{Blue, Cyan, Green, Red, Yellow};
use itertools::Itertools;
use std::collections::HashSet;
use std::io::{self, Write};
use std::process::Command;

use crate::config::get_packages;

pub fn show_diff() -> Result<(), std::io::Error> {
	let explicitly_installed = get_explicitly_installed_packages()?;
	let explicitly_installed_set: HashSet<String> = explicitly_installed.into_iter().collect();

	let all_installed = get_installed_packages()?;
	let all_installed_set: HashSet<String> = all_installed.into_iter().collect();

	let managed = get_managed_packages()?;

	let missing: Vec<&String> = managed.difference(&all_installed_set).sorted().collect();
	let extra: Vec<&String> = explicitly_installed_set.difference(&managed).sorted().collect();

	println!("{}", Blue.paint("📦 Package Status Overview"));
	println!();

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

	if missing.is_empty() && extra.is_empty() {
		println!("{}", Green.paint("✅ All packages are in sync!"));
	}

	Ok(())
}

pub fn install() -> Result<(), std::io::Error> {
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
	let status = Command::new("paru")
		.args(["-S", "--needed", "--noconfirm"])
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

fn get_managed_packages() -> Result<HashSet<String>, std::io::Error> {
	let packages_config = get_packages()?;
	let mut managed = HashSet::new();

	for (_, package_list) in packages_config {
		managed.extend(package_list);
	}

	Ok(managed)
}

fn get_installed_packages() -> Result<Vec<String>, std::io::Error> {
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

fn get_explicitly_installed_packages() -> Result<Vec<String>, std::io::Error> {
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
