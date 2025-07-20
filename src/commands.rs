use std::{fs, os::unix::fs::symlink, path::Path};

pub fn link(original_path: &Path, symlink_target_path: &Path) -> Result<(), std::io::Error> {
	if symlink_target_path.exists() {
		if let Ok(existing_target) = fs::read_link(symlink_target_path) {
			if existing_target == original_path {
				println!("✅ Already linked: {symlink_target_path:?} -> {original_path:?}");
				return Ok(());
			}
		}
	}

	if let Some(parent_dir) = symlink_target_path.parent() {
		fs::create_dir_all(parent_dir)?;
	}

	if let Err(error) = symlink(original_path, symlink_target_path) {
		if error.kind() == std::io::ErrorKind::AlreadyExists {
			println!("❌ File already exists: {symlink_target_path:?}");
			return Ok(());
		}

		Err(error)?
	}

	println!("✅ Symlink created: {symlink_target_path:?} -> {original_path:?}");
	Ok(())
}

pub fn unlink(symlink_target_path: &Path) -> Result<(), std::io::Error> {
	if symlink_target_path.exists() {
		fs::remove_file(symlink_target_path)?;
	}

	println!("✅ Deleted: {symlink_target_path:?}");
	Ok(())
}
