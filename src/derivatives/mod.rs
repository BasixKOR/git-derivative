use std::collections::BTreeMap;

use relative_path::RelativePathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivativeConfig {
    pub generators: BTreeMap<RelativePathBuf, String>,
}

pub mod file;
pub mod updater;
