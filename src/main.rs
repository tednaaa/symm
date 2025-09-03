use ansi_term::Colour::Red;
use clap::{Parser, Subcommand};
use config::ConfigPaths;

mod cli;
mod config;
mod packages;
mod symlinks;

#[derive(Parser)]
#[command(version, about = "Simple symlink and package manager", long_about = None)]
pub struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
	/// Manage symlinks
	Link,
	Unlink,

	/// Manage packages
	Packages(PackagesCommands),
}

#[derive(Subcommand)]
pub enum PackagesCommands {
	/// Show missing packages
	Diff,

	/// Install missing packages
	Install {
		/// Skip confirmation prompt
		#[arg(long)]
		noconfirm: bool,
	},
}

fn main() -> Result<(), std::io::Error> {
	let cli = Cli::parse();

	match cli.command {
		Commands::Link => {
			let symlinks = config::get_symlinks()?;
			let ConfigPaths { home_dir, configs_dir, .. } = config::get_config_paths()?;

			for (key, value) in symlinks {
				let original_path = configs_dir.join(key);
				let symlink_target_path = home_dir.join(value);

				symlinks::link(&original_path, &symlink_target_path)?
			}
		},
		Commands::Unlink => {
			let symlinks = config::get_symlinks()?;
			let ConfigPaths { home_dir, .. } = config::get_config_paths()?;

			for value in symlinks.values() {
				let symlink_target_path = home_dir.join(value);

				symlinks::unlink(&symlink_target_path)?
			}
		},
	}

	if let Some(command) = cli.command {
		match command {
			"unlink" => {},
			"packages" => match cli.subcommand.as_deref() {
				Some("diff") => packages::show_diff()?,
				Some("install") => packages::install()?,
				Some(subcommand) => {
					eprintln!("{}", Red.paint(format!("Subcommand {subcommand} does not exist.")))
				},
				None => eprintln!("{}", Red.paint("Missing subcommand.")),
			},
			_ => eprintln!("{}", Red.paint(format!("Command {command} does not exist."))),
		}
	}

	Ok(())
}
