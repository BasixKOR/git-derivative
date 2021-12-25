use std::path::Path;

use clap::Parser;
use git_derivative::args::{Cli, Subcommands};
use git_derivative::derivatives::file::{
    create_file, find_file, get_git_repository_path, parse_from_file,
};
use git_derivative::derivatives::updater::{run_config, run_all_config};
use git_derivative::git::get_changed_files;
use git_derivative::hook::install_hook;
use relative_path::RelativePath;
use miette::{IntoDiagnostic, Result};

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.subcommand {
        Subcommands::Init => {
            create_file(Path::new(".")).into_diagnostic()?;
        }
        Subcommands::Install => match get_git_repository_path(Path::new(".")) {
            Some(path) => install_hook(&path)?,
            None => println!("Not a git repository"),
        },
        Subcommands::Update => {
            let config_file = find_file(Path::new("."))?;
            let (config, source) = parse_from_file(&config_file)?;
            if let Some(root) = get_git_repository_path(Path::new(".")) {
                run_all_config(&config, &root, source)?;
            } else {
                println!("Not a git repository");
            }
        }
        Subcommands::Hook { prev, current, .. } => {
            let files = get_changed_files(&prev, &current).into_diagnostic()?;
            let file_paths = files
                .iter()
                .map(RelativePath::new)
                .collect::<Vec<_>>();
            let config_file = find_file(Path::new("."))?;
            let (config, source) = parse_from_file(&config_file)?;
            if let Some(root) = get_git_repository_path(Path::new(".")) {
                run_config(&config, &root, &file_paths[..], source)?;
            } else {
                println!("Not a git repository");
            }
        }
    }

    Ok(())
}
