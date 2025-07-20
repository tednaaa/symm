use clap::Parser;
use config::ConfigPaths;

mod commands;
mod config;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
	/// link or unlink
	command: Option<String>,
}

fn main() -> Result<(), std::io::Error> {
	let cli = Cli::parse();

	let symlinks = config::get_symlinks()?;
	let ConfigPaths { home_dir, configs_dir, .. } = config::get_config_paths()?;

	if let Some(command) = cli.command.as_deref() {
		match command {
			"link" => {
				for (key, value) in symlinks {
					let original_path = configs_dir.join(key);
					let symlink_target_path = home_dir.join(value);

					commands::link(&original_path, &symlink_target_path)?
				}
			},
			"unlink" => {
				for value in symlinks.values() {
					let symlink_target_path = home_dir.join(value);

					commands::unlink(&symlink_target_path)?
				}
			},
			_ => println!("Such command does not exist"),
		}
	}

	Ok(())
}
