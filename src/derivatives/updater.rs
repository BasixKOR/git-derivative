use relative_path::RelativePath;
use serde::Serialize;
use std::{path::Path, process::Command};
use tinytemplate::TinyTemplate;
use thiserror::Error;
use miette::Diagnostic;

use super::DerivativeConfig;

#[derive(Serialize)]
struct Context<'a> {
    path: &'a RelativePath,
}

#[derive(Error, Diagnostic, Debug)]
pub enum UpdateError {
    #[error(transparent)]
    #[diagnostic(code(tinytemplate::error::Error))]
    TemplateError(#[from] tinytemplate::error::Error),
    #[error(transparent)]
    #[diagnostic(code(std::io::Error))]
    IoError(#[from] std::io::Error),
}

pub fn run_config(
    config: &DerivativeConfig,
    root: &Path,
    requested_paths: &[&RelativePath],
) -> Result<bool, UpdateError> {
    let mut success = true;
    for &path in requested_paths {
        let mut template = TinyTemplate::new();

        let generator = if let Some(v) = config.generators.get(path) {
            v
        } else {
            continue;
        };

        template.add_template(path.as_str(), generator)?;

        let command = template.render(path.as_str(), &Context { path })?;

        #[cfg(not(target_os = "windows"))]
        let result = Command::new("sh")
            .arg("-c")
            .arg(command)
            .current_dir(root)
            .status()?;
        #[cfg(target_os = "windows")]
        let result = Command::new("cmd")
            .arg("/C")
            .arg(command)
            .current_dir(root)
            .status()?;

        if !result.success() {
            success = false;
        }
    }
    Ok(success)
}

pub fn run_all_config(config: &DerivativeConfig, root: &Path) -> Result<bool, UpdateError> {
    let mut success = true;
    for (path, generator) in config.generators.iter() {
        let mut template = TinyTemplate::new();
        template.add_template(path.as_str(), generator)?;

        let command = template.render(path.as_str(), &Context { path })?;

        #[cfg(not(target_os = "windows"))]
        let result = Command::new("sh")
            .arg("-c")
            .arg(command)
            .current_dir(root)
            .status()?;
        #[cfg(target_os = "windows")]
        let result = Command::new("cmd")
            .arg("/C")
            .arg(command)
            .current_dir(root)
            .status()?;

        if !result.success() {
            success = false;
        }
    }
    Ok(success)
}
