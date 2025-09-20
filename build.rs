#[path = "src/cli.rs"]
mod cli;

use std::{env, fs, io, path::PathBuf};

use clap::CommandFactory;
use clap_complete::{generate_to, Shell};
use cli::Cli;

fn main() -> io::Result<()> {
	// I'm not sure is this needed or not
	// println!("cargo:rerun-if-changed=build.rs");
	// println!("cargo:rerun-if-changed=src/");

	generate_completions()
}

fn generate_completions() -> io::Result<()> {
	let target_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target");
	let completions_dir = target_dir.join("completions");

	let mut cmd = Cli::command();
	let bin_name = env!("CARGO_PKG_NAME");

	fs::create_dir_all(&completions_dir)?;

	for shell in [Shell::Bash, Shell::Fish, Shell::Zsh] {
		generate_to(shell, &mut cmd, bin_name, &completions_dir)?;
	}

	Ok(())
}
