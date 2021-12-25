use std::collections::BTreeMap;

use relative_path::RelativePathBuf;
use serde::{Deserialize, Serialize};
use toml::Spanned;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivativeConfig {
    pub generators: BTreeMap<RelativePathBuf, Spanned<String>>,
}

pub mod file;
pub mod updater;
