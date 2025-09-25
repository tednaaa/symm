use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about = "Simple dotfiles manager", long_about = None)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
	/// Create symlinks
	Link,
	/// Remove symlinks
	Unlink,
	/// Manage packages
	#[command(subcommand)]
	Packages(PackagesCommands),
}

#[derive(Subcommand)]
pub enum PackagesCommands {
	/// Show missing packages
	Diff {
		/// Show ignored packages
		#[arg(long)]
		ignored: bool,
	},
	/// Install missing packages
	Install {
		/// Skip confirmation prompt
		#[arg(long)]
		noconfirm: bool,
	},
}
