
use clap::{Parser, Subcommand};

/// git-derivative is an applications to manage artifacts derived from a git-managed files.
/// It can be used for dependency management and automatic installation etc.
#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Cli {
	#[clap(subcommand)]
	pub subcommand: Subcommands,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
	/// Initializes a git-derivative repository.
	Init,
	/// Install a post-checkout hook.
	Install,
	/// Update files. Can be called by hook or manually.
	Update {
		/// Forcefully update all files, regardless of their modification.
		#[clap(short, long)]
		force: bool,
	},
}