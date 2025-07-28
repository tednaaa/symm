use ansi_term::Colour::{Black, Cyan, Green, Red};
use std::{
	fs,
	os::unix::fs::symlink,
	path::{Path, PathBuf},
};

use symm::prettify_path;

pub fn link(original_path: &Path, symlink_target_path: &Path) -> Result<(), std::io::Error> {
	let pretty_symlink_target_path = prettify_path(symlink_target_path);
	let pretty_original_path = prettify_path(original_path);

	if symlink_target_path.exists() {
		if let Ok(existing_target) = fs::read_link(symlink_target_path) {
			if existing_target == original_path {
				println!(
					"✅ Already linked: {} {} {}",
					Cyan.paint(pretty_original_path),
					Black.paint("→"),
					Green.paint(pretty_symlink_target_path)
				);
				return Ok(());
			}
		}
	}

	if let Some(parent_dir) = symlink_target_path.parent() {
		fs::create_dir_all(parent_dir)?;
	}

	if let Err(error) = symlink(original_path, symlink_target_path) {
		if error.kind() == std::io::ErrorKind::AlreadyExists {
			println!("❌ File/Directory already exists {} {}", Black.paint("→"), Red.paint(pretty_symlink_target_path));
			return Ok(());
		}

		Err(error)?
	}

	println!(
		"✅ Symlink created: {} {} {}",
		Cyan.paint(pretty_original_path),
		Black.paint("→"),
		Green.paint(pretty_symlink_target_path)
	);
	Ok(())
}

pub fn unlink(symlink_target_path: &Path) -> Result<(), std::io::Error> {
	if !symlink_target_path.exists() {
		return Ok(());
	}

	if symlink_target_path.is_file() {
		fs::remove_file(symlink_target_path)?;
	} else if symlink_target_path.is_dir() {
		fs::remove_dir_all(symlink_target_path)?;
	}

	let pretty_symlink_target_path = prettify_path(symlink_target_path);
	println!("✅ Deleted: {}", Red.paint(pretty_symlink_target_path));

	Ok(())
}
