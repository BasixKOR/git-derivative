use std::path::Path;

use clap::Parser;
use color_eyre::eyre::Result;
use git_derivative::args::{Cli, Subcommands};
use git_derivative::derivatives::file::{
    create_file, find_file, get_git_repository_path, parse_from_file,
};
use git_derivative::derivatives::updater::run_config;
use git_derivative::git::get_changed_files;
use git_derivative::hook::install_hook;
use relative_path::RelativePath;

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
        Subcommands::Hook { prev, current, .. } => {
            let files = get_changed_files(&prev, &current)?;
            let file_paths = files
                .iter()
                .map(|file| RelativePath::new(file))
                .collect::<Vec<_>>();
            let config_file = find_file(Path::new("."))?;
            let config = parse_from_file(&config_file)?;
            if let Some(root) = get_git_repository_path(Path::new(".")) {
                run_config(&config, &root, &file_paths[..])?;
            } else {
                println!("Not a git repository");
            }
        }
    }

    Ok(())
}
