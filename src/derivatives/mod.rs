use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivativeConfig {
    #[serde(with = "indexmap::serde_seq")]
    pub generators: IndexMap<String, String>,
}

pub mod file;
pub mod updater;
