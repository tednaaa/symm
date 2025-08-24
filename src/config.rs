use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

const DOTFILES_DIR: &str = "dotfiles";
const CONFIG_FILE: &str = "dotfiles.toml";
const CONFIGS_DIR: &str = ".configs";

pub struct ConfigPaths {
	pub home_dir: PathBuf,
	pub configs_dir: PathBuf,
	pub config_path: PathBuf,
}

#[derive(Deserialize)]
pub struct Config {
	pub symlinks: Option<HashMap<String, String>>,
	pub packages: Option<HashMap<String, Vec<String>>>,
}

pub fn get_base_packages() -> Vec<String> {
	vec![
		"base".to_string(),
		"base-devel".to_string(),
		"linux".to_string(),
		"linux-firmware".to_string(),
		"efibootmgr".to_string(),
		"intel-ucode".to_string(),
		"btrfs-progs".to_string(),
		"snapper".to_string(),
	]
}

pub fn get_system_packages() -> Vec<String> {
	vec![
		"networkmanager".to_string(),
		"pipewire".to_string(),
		"pipewire-alsa".to_string(),
		"pipewire-jack".to_string(),
		"pipewire-pulse".to_string(),
		"wireplumber".to_string(),
		"libpulse".to_string(),
		"gst-plugin-pipewire".to_string(),
		"pavucontrol".to_string(),
		"bluez".to_string(),
		"bluez-utils".to_string(),
		"sof-firmware".to_string(),
		"zram-generator".to_string(),
		"git".to_string(),
		"fish".to_string(),
	]
}

pub fn get_config_paths() -> Result<ConfigPaths, std::io::Error> {
	if let Some(home_dir) = dirs::home_dir() {
		let dotfiles_dir = home_dir.join(DOTFILES_DIR);
		let configs_dir = dotfiles_dir.join(CONFIGS_DIR);
		let config_path = dotfiles_dir.join(CONFIG_FILE);

		return Ok(ConfigPaths { home_dir, configs_dir, config_path });
	}

	Err(io::Error::new(io::ErrorKind::NotFound, "No home directory found"))
}

pub fn get_config() -> Result<Config, std::io::Error> {
	let ConfigPaths { config_path, .. } = get_config_paths()?;

	let mut file = File::open(config_path)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;

	let config: Config = toml::from_str(&contents).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;

	Ok(config)
}

pub fn get_symlinks() -> Result<HashMap<String, String>, std::io::Error> {
	let config = get_config()?;

	if let Some(symlinks) = config.symlinks {
		Ok(symlinks)
	} else {
		Err(io::Error::new(io::ErrorKind::NotFound, "Missing [symlinks] section in config"))
	}
}

pub fn get_packages() -> Result<HashMap<String, Vec<String>>, std::io::Error> {
	let config = get_config()?;

	if let Some(packages) = config.packages {
		Ok(packages)
	} else {
		Ok(HashMap::new())
	}
}
