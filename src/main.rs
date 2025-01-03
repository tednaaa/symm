use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::{env, fs, os::unix::fs::symlink, path::Path};

fn main() -> Result<(), std::io::Error> {
	if let Some(home) = dirs::home_dir() {
		let dotfiles_dir = home.join("dotfiles");

		let mut file = File::open("dotfiles.toml")?;
		let mut contents = String::new();
		file.read_to_string(&mut contents)?;

		let config: HashMap<String, HashMap<String, String>> =
			toml::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
		let symlinks = config.get("symlinks").expect("Missing [symlinks] section in config");

		let args: Vec<String> = env::args().collect();
		if args.len() < 2 {
			panic!("Usage: {} <command>", args[0]);
		}
		let command = args[1].as_str();

		match command {
			"link" => {
				for (key, value) in symlinks {
					let target = dotfiles_dir.join(key);
					let link_name = home.join(value);

					link(&target, &link_name)?;
				}
			}
			"unlink" => {
				for value in symlinks.values() {
					let link_name = home.join(value);
					unlink(&link_name)?;
				}
			}
			_ => panic!("{command}: command not found"),
		}
	}

	Ok(())
}

fn link(original_path: &Path, symlink_target_path: &Path) -> Result<(), std::io::Error> {
	if symlink_target_path.exists() {
		if let Ok(existing_target) = fs::read_link(symlink_target_path) {
			if existing_target == original_path {
				println!("✅ Already linked: {:?} -> {:?}", symlink_target_path, original_path);
				return Ok(());
			}
		}
	}

	if let Some(parent_dir) = symlink_target_path.parent() {
		fs::create_dir_all(parent_dir)?;
	}

	if let Err(error) = symlink(original_path, symlink_target_path) {
		if error.kind() != std::io::ErrorKind::AlreadyExists {
			Err(error)?;
		}

		println!("❌ File already exists: {:?}", symlink_target_path);
		return Ok(());
	}

	println!("✅ Symlink created: {:?} -> {:?}", symlink_target_path, original_path);
	Ok(())
}

fn unlink(symlink_target_path: &Path) -> Result<(), std::io::Error> {
	if symlink_target_path.exists() {
		fs::remove_file(symlink_target_path)?;
	}

	println!("✅ Deleted: {:?}", symlink_target_path);
	Ok(())
}
