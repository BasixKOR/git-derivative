use color_eyre::eyre::Result;
use relative_path::RelativePath;
use serde::Serialize;
use std::{path::Path, process::Command};
use tinytemplate::TinyTemplate;

use super::DerivativeConfig;

#[derive(Serialize)]
struct Context<'a> {
    path: &'a RelativePath,
}

pub fn run_config(
    config: &DerivativeConfig,
    root: &Path,
    requested_paths: &[&RelativePath],
) -> Result<bool> {
    let mut success = true;
    for &path in requested_paths {
        let mut template = TinyTemplate::new();

        let generator = if let Some(v) = config.generators.get(path) {
            v
        } else {
            continue;
        };

        template.add_template(path.as_str(), &generator)?;

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

pub fn run_all_config(config: &DerivativeConfig, root: &Path) -> Result<bool> {
    let mut success = true;
    for (path, generator) in config.generators.iter() {
        let mut template = TinyTemplate::new();
        template.add_template(path.as_str(), &generator)?;

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
