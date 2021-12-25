use std::fs::File;
use std::io::{Error as IoError, Read, Result as IoResult};
use std::path::{Path, PathBuf};

use miette::{Diagnostic, NamedSource, SourceOffset, SourceSpan};
use thiserror::Error;

use super::DerivativeConfig;

pub fn get_git_repository_path(path: &Path) -> Option<PathBuf> {
    for folder in path.ancestors() {
        let git_path = folder.join(".git");
        if git_path.exists() {
            return Some(folder.to_owned());
        }
    }
    None
}

#[derive(Error, Diagnostic, Debug)]
pub enum FindError {
    #[error(transparent)]
    #[diagnostic(code(std::io::Error))]
    IoError(#[from] IoError),

    #[error("No config file found")]
    #[diagnostic(
        code(git_derivative::derivatives::file::FindError::NoConfig),
        help("try running `git derivative init`")
    )]
    NoConfig,

    #[error("No git repository found")]
    #[diagnostic(
        code(git_derivative::derivatives::file::FindError::NoGitRepository),
        help("try running `git init`")
    )]
    NoGitRepository,
}

/// Finds a .gitderivative file in the given Git repository and returns its path.
pub fn find_file(path: &Path) -> Result<PathBuf, FindError> {
    let folder = get_git_repository_path(path).ok_or(FindError::NoGitRepository)?;
    let file = folder.join(".gitderivative");
    if file.exists() {
        return Ok(file);
    }
    Err(FindError::NoConfig)
}

#[derive(Error, Debug, Diagnostic)]

pub enum ParseError {
    #[error(transparent)]
    #[diagnostic(code(std::io::Error))]
    Io(#[from] IoError),

    #[error("Invalid TOML")]
    #[diagnostic(code(toml::de::Error::syntax))]
    TomlSyntax {
        source: toml::de::Error,
        #[source_code]
        source_code: NamedSource,
        #[label]
        span: SourceSpan,
    },

    #[error(transparent)]
    #[diagnostic(code(toml::de::Error))]
    Toml(#[from] toml::de::Error),
}

pub fn parse_from_file(path: &Path) -> Result<(DerivativeConfig, String), ParseError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok((
        toml::from_str(&contents).or_else(|source| {
            if let Some((line, col)) = source.line_col() {
                let start = SourceOffset::from_location(&contents, line, col);
                let len = crate::derivatives::to_source_offset(0);
                Err(ParseError::TomlSyntax {
                    source,
                    source_code: NamedSource::new(
                        path.file_name().unwrap().to_string_lossy(),
                        contents.clone(),
                    ),
                    span: (start, len).into(),
                })
            } else {
                Err(ParseError::Toml(source))
            }
        })?,
        contents,
    ))
}

pub fn create_file(path: &Path) -> IoResult<File> {
    File::create(path.join(".gitderivative"))
}
