use color_eyre::eyre::Result;
use serde::Serialize;
use std::{path::Path, process::Command};
use tinytemplate::TinyTemplate;

use super::DerivativeConfig;

#[derive(Serialize)]
struct Context<'a> {
    path: &'a Path,
}

// TODO: Make paths match regardless of path style (absolute, relative, etc.)
pub fn run_config(
    config: &DerivativeConfig,
    requested_paths: &[&Path],
) -> Result<bool> {
    let mut success = true;
    for &path in requested_paths {
        let mut template = TinyTemplate::new();

        let generator = if let Some(v) = config.generators.get(path) {
            v
        } else {
            continue;
        };

        let path_string = path.to_string_lossy();
        template.add_template(&path_string, &generator)?;

        let path = Path::new(path);
        let command = template.render(&path_string, &Context { path })?;

        #[cfg(not(target_os = "windows"))]
        let result = Command::new("sh")
            .arg("-c")
            .arg(command)
            .current_dir(path)
            .status()?;
        #[cfg(target_os = "windows")]
        let result = Command::new("cmd")
            .arg("/C")
            .arg(command)
            .current_dir(path)
            .status()?;

        if !result.success() {
            success = false;
        }
    }
    Ok(success)
}
