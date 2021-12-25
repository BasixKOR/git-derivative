use std::path::Path;

use clap::Parser;
use color_eyre::eyre::Result;
use git_derivative::args::{Cli, Subcommands};
use git_derivative::derivatives::file::{create_file, get_git_repository_path};
use git_derivative::hook::install_hook;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Cli::parse();

    match args.subcommand {
        Subcommands::Init => {
            create_file(Path::new("."))?;
        }
        Subcommands::Install => match get_git_repository_path(Path::new(".")) {
            Some(path) => install_hook(&path)?,
            None => println!("Not a git repository"),
        },
        Subcommands::Update { force } => {
            println!("update");
            println!("force: {}", force);
        }
    }

    Ok(())
}
