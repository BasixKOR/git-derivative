use color_eyre::eyre::Result;
use globwalk::{FileType, GlobWalkerBuilder};
use serde::Serialize;
use std::{path::Path, process::Command};
use tinytemplate::TinyTemplate;

use super::DerivativeConfig;

#[derive(Serialize)]
struct Context<'a> {
    path: &'a Path,
}

pub fn run_config(config: &DerivativeConfig, root: &Path) -> Result<bool> {
    let mut template = TinyTemplate::new();
    let mut success = true;
    for (path, generator) in &config.generators {
        template.add_template(&path, &generator)?;

        let walker = GlobWalkerBuilder::new(root, path)
            .file_type(FileType::FILE)
            .build()?;

        for entry in walker.filter_map(|e| e.ok()) {
            let command = template.render(&path, &Context { path: entry.path() })?;

            #[cfg(not(target_os = "windows"))]
            let result = Command::new("sh")
                .arg("-c")
                .arg(command)
                .current_dir(entry.path())
                .status()?;
            #[cfg(target_os = "windows")]
            let result = Command::new("cmd")
                .arg("/C")
                .arg(command)
                .current_dir(entry.path())
                .status()?;
            
            if !result.success() {
                success = false;
            }
        }
    }
    Ok(success)
}
