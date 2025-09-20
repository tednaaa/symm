use ansi_term::Colour::Red;
use clap::Parser;
use config::ConfigPaths;

mod cli;
mod config;
mod packages;
mod symlinks;

use cli::{Cli, Commands, PackagesCommands};

fn main() -> Result<(), std::io::Error> {
	let cli = Cli::parse();

	match cli.command {
		Commands::Link => {
			let symlinks = config::get_symlinks()?;
			let ConfigPaths { home_dir, configs_dir, .. } = config::get_config_paths()?;

			for (key, value) in symlinks {
				let original_path = configs_dir.join(key);
				let symlink_target_path = home_dir.join(value);

				symlinks::link(&original_path, &symlink_target_path)?;
			}
		},
		Commands::Unlink => {
			let symlinks = config::get_symlinks()?;
			let ConfigPaths { home_dir, .. } = config::get_config_paths()?;

			for value in symlinks.values() {
				let symlink_target_path = home_dir.join(value);
				symlinks::unlink(&symlink_target_path)?;
			}
		},
		Commands::Packages(packages_cmd) => match packages_cmd {
			PackagesCommands::Diff => match packages::show_diff() {
				Ok(_) => {},
				Err(e) => eprintln!("{}", Red.paint(format!("Failed to show package diff: {}", e))),
			},
			PackagesCommands::Install { noconfirm } => match packages::install(noconfirm) {
				Ok(_) => {},
				Err(e) => eprintln!("{}", Red.paint(format!("Failed to install packages: {}", e))),
			},
		},
	}

	Ok(())
}
