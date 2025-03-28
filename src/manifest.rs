use anyhow::{Context, Result};
use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

#[derive(Debug, Deserialize, PartialEq)]
pub struct Manifest {
    pub env: Option<HashMap<String, String>>,
    pub dotfiles: Option<HashMap<String, DotfileEntry>>,
    pub external: Option<HashMap<String, ExternalEntry>>,
    pub packages: Option<HashMap<String, Vec<String>>>,
    pub options: Option<Options>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DotfileEntry {
    pub source: String,
    pub target: String,
    #[serde(default)]
    pub method: DotfileMethod,
    #[serde(default)]
    pub backup: bool,
}

#[derive(Default, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DotfileMethod {
    #[default]
    Symlink,
    Copy,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ExternalEntry {
    pub method: ExternalMethod,
    pub source: String,
    pub target: String,
    #[serde(default)]
    pub extract: Option<ExtractOptions>,
    #[serde(default)]
    pub checksum: Option<String>,
    #[serde(default)]
    pub timeout: Option<u32>,
    #[serde(default)]
    pub branch: Option<String>,
    #[serde(default)]
    pub depth: Option<u32>,
    #[serde(default)]
    pub recurse_submodules: Option<bool>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExternalMethod {
    Download,
    Git,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ExtractOptions {
    pub format: String,
    #[serde(default)]
    pub strip_components: Option<u32>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Options {
    #[serde(default)]
    pub overwrite: bool,
    #[serde(default)]
    pub backup: bool,
}

pub fn load_manifest<P: AsRef<Path>>(path: P) -> Result<Manifest> {
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read manifest file: {}", path.as_ref().display()))?;

    let manifest: Manifest =
        toml::from_str(&content).with_context(|| "Failed to parse manifest.toml")?;

    Ok(manifest)
}
