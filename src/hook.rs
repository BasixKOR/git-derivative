use std::io::Write;
use std::fs::File;
use std::path::Path;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HookInstallationError {
    #[error("The hook is already installed")]
    AlreadyInstalled,
    #[error("Failed to write file: {0:?}")]
    WriteError(#[from] std::io::Error),
}

pub fn install_hook(path: &Path) -> Result<(), HookInstallationError> {
    let hook_path = path.join(".git/hooks/post-checkout");
    if hook_path.exists() {
        return Err(HookInstallationError::AlreadyInstalled);
    }
    let mut file = File::create(hook_path).unwrap();
	file.write_all(include_bytes!("../resources/hook.sh"))?;
    file.sync_all()?;
	Ok(())
}
