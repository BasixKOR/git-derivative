use std::collections::BTreeMap;

use relative_path::RelativePathBuf;
use serde::{Deserialize, Serialize};
use toml::Spanned;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivativeConfig {
    pub generators: BTreeMap<RelativePathBuf, Spanned<String>>,
}

// https://github.com/zkat/miette/issues/98
fn to_source_offset(num: usize) -> miette::SourceOffset {
    unsafe { std::mem::transmute(num) }
}

pub mod file;
pub mod updater;
