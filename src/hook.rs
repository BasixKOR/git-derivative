use std::fs::{File, Permissions};
use std::io::Write;
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

    #[cfg(unix)] // Give execute permissions to the hook
    {
        use std::os::unix::fs::PermissionsExt;
        file.set_permissions(Permissions::from_mode(0o555))?;
    }

    file.sync_all()?;
    Ok(())
}
