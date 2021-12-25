use std::path::Path;

use clap::Parser;
use git_derivative::args::{Cli, Subcommands};
use git_derivative::derivatives::file::{
    create_file, find_file, get_git_repository_path, parse_from_file,
};
use git_derivative::derivatives::updater::{run_all_config, run_config};
use git_derivative::git::get_changed_files;
use git_derivative::hook::install_hook;
use miette::{Diagnostic, IntoDiagnostic, Result};
use relative_path::RelativePath;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
#[error("Couldn't find a git repository at the location.")]
#[diagnostic(
    code(git_derivative::NotGitRepositoryError),
    help("Did you forget to run `git init`?")
)]
struct NotGitRepositoryError();

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.subcommand {
        Subcommands::Init => match get_git_repository_path(Path::new(".")) {
            Some(root) => Ok(create_file(&root).map(|_| ()).into_diagnostic()?),
            None => Err(NotGitRepositoryError().into()),
        },
        Subcommands::Install => match get_git_repository_path(Path::new(".")) {
            Some(path) => Ok(install_hook(&path)?),
            None => Err(NotGitRepositoryError().into()),
        },
        Subcommands::Update => {
            let config_file = find_file(Path::new("."))?;
            let (config, source) = parse_from_file(&config_file)?;
            if let Some(root) = get_git_repository_path(Path::new(".")) {
                Ok(run_all_config(&config, &root, source)?)
            } else {
                Err(NotGitRepositoryError().into())
            }
        }
        Subcommands::Hook { prev, current, .. } => {
            let files = get_changed_files(&prev, &current).into_diagnostic()?;
            let file_paths = files.iter().map(RelativePath::new).collect::<Vec<_>>();
            let config_file = find_file(Path::new("."))?;
            let (config, source) = parse_from_file(&config_file)?;
            if let Some(root) = get_git_repository_path(Path::new(".")) {
                Ok(run_config(&config, &root, &file_paths[..], source)?)
            } else {
                Err(NotGitRepositoryError().into())
            }
        }
    }
}
