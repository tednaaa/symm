use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

const CONFIG_DIR: &str = "dotfiles";
const CONFIG_FILE: &str = "dotfiles.toml";

pub struct ConfigPaths {
	pub home_dir: PathBuf,
	pub dotfiles_dir: PathBuf,
	pub config_path: PathBuf,
}

pub fn get_config_paths() -> Result<ConfigPaths, std::io::Error> {
	if let Some(home_dir) = dirs::home_dir() {
		let dotfiles_dir = home_dir.join(CONFIG_DIR);
		let config_path = dotfiles_dir.join(CONFIG_FILE);

		return Ok(ConfigPaths { home_dir, dotfiles_dir, config_path });
	}

	Err(io::Error::new(io::ErrorKind::NotFound, "No home directory found"))
}

pub fn get_symlinks() -> Result<HashMap<String, String>, std::io::Error> {
	let ConfigPaths { config_path, .. } = get_config_paths()?;

	let mut file = File::open(config_path)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;

	let config: HashMap<String, HashMap<String, String>> =
		toml::from_str(&contents).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;

	if let Some(symlinks) = config.get("symlinks") {
		Ok(symlinks.clone())
	} else {
		Err(io::Error::new(io::ErrorKind::NotFound, "Missing [symlinks] section in config"))
	}
}
