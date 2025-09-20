#[path = "src/cli.rs"]
mod cli;

use std::{env, io};

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
	let completion_dir = "completions";
	std::fs::create_dir_all(completion_dir)?;

	let mut cmd = Cli::command();
	let bin_name = env!("CARGO_PKG_NAME");

	for shell in [Shell::Bash, Shell::Fish, Shell::Zsh] {
		generate_to(shell, &mut cmd, bin_name, completion_dir)?;
	}

	Ok(())
}
