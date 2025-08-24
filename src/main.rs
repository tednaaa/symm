use ansi_term::Colour::Red;
use clap::Parser;
use config::ConfigPaths;

mod commands;
mod config;
mod packages;

#[derive(Parser)]
#[command(version, about = "Simple symlink and package manager", long_about = None)]
struct Cli {
	/// Command to run: 'link' | 'unlink' | 'packages'
	command: Option<String>,
	/// Subcommand for packages: 'diff' | 'install'
	subcommand: Option<String>,
}

fn main() -> Result<(), std::io::Error> {
	let cli = Cli::parse();

	if let Some(command) = cli.command.as_deref() {
		match command {
			"link" => {
				let symlinks = config::get_symlinks()?;
				let ConfigPaths { home_dir, configs_dir, .. } = config::get_config_paths()?;

				for (key, value) in symlinks {
					let original_path = configs_dir.join(key);
					let symlink_target_path = home_dir.join(value);

					commands::link(&original_path, &symlink_target_path)?
				}
			},
			"unlink" => {
				let symlinks = config::get_symlinks()?;
				let ConfigPaths { home_dir, .. } = config::get_config_paths()?;

				for value in symlinks.values() {
					let symlink_target_path = home_dir.join(value);

					commands::unlink(&symlink_target_path)?
				}
			},
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
