use std::fs::File;
use std::io::{Error as IoError, ErrorKind as IoErrorKind, Read, Result as IoResult};
use std::path::{Path, PathBuf};

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

/// Finds a .gitderivative file in the given Git repository and returns its path.
pub fn find_file(path: &Path) -> IoResult<PathBuf> {
    let folder = get_git_repository_path(path).ok_or(IoError::new(
        IoErrorKind::NotFound,
        "No Git repository found",
    ))?;
    let file = folder.join(".gitderivative");
    if file.exists() {
        return Ok(file.to_owned());
    }
    Err(IoError::new(
        IoErrorKind::NotFound,
        "No .gitderivative file found",
    ))
}

pub fn parse_from_file(path: &Path) -> IoResult<DerivativeConfig> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(toml::from_str(&contents)?)
}

pub fn create_file(path: &Path) -> IoResult<File> {
    File::create(path.join(".gitderivative"))
}
