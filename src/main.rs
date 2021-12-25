use std::path::Path;

use clap::Parser;
use git_derivative::{
    args::{Cli, Subcommands},
    derivatives::file::create_file,
};
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Cli::parse();

    match args.subcommand {
        Subcommands::Init => {
            create_file(Path::new("."))?;
        }
        Subcommands::Install => {
            println!("install");
        }
        Subcommands::Update { force } => {
            println!("update");
            println!("force: {}", force);
        }
    }

    Ok(())
}
