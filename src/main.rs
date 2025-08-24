use ansi_term::Colour::Red;
use clap::Parser;
use config::ConfigPaths;

mod commands;
mod config;
mod packages;

#[derive(Parser)]
#[command(version, about = "Simple symlink and package manager", long_about = None)]
struct Cli {
	/// Command to run: 'link' (create symlinks), 'unlink' (remove symlinks), 'packages' (manage packages)
	command: Option<String>,
	/// Subcommand for packages: 'diff' (show differences), 'sync' (install missing), 'export' (list installable packages)
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
				Some("diff") => packages::packages_diff()?,
				Some("sync") => packages::packages_sync()?,
				Some("export") => packages::packages_export()?,
				Some(subcommand) => {
					eprintln!("{}", Red.paint(format!("Subcommand {subcommand} does not exist. Use: diff, sync, export")))
				},
				None => eprintln!("{}", Red.paint("Missing subcommand. Use: packages <diff|sync|export>")),
			},
			_ => eprintln!("{}", Red.paint(format!("Command {command} does not exist. Use: link, unlink, packages"))),
		}
	}

	Ok(())
}
