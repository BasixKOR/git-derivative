use indexmap::IndexMap;
use relative_path::RelativePathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivativeConfig {
    #[serde(with = "indexmap::serde_seq")]
    pub generators: IndexMap<RelativePathBuf, String>,
}

pub mod file;
pub mod updater;
