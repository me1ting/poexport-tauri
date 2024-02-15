use std::{fs, path::PathBuf};

use anyhow::{bail, Context, Result};
use serde::{de::DeserializeOwned, Serialize};

pub fn read_json<T: DeserializeOwned>(path: &PathBuf) -> Result<T> {
    if !path.exists() {
        bail!("file not found \"{}\"", path.display());
    }
    let yaml_str = fs::read_to_string(&path)
        .with_context(|| format!("failed to read the file \"{}\"", path.display()))?;

    serde_json::from_str::<T>(&yaml_str).with_context(|| {
        format!(
            "failed to read the file with yaml format \"{}\"",
            path.display()
        )
    })
}

pub fn write_json<T: Serialize>(path: &PathBuf, data: &T) -> Result<()> {
    let yaml_str = serde_json::to_string(data).context("failed to serialize data")?;
    fs::write(path, yaml_str)
        .with_context(|| format!("failed to write the file \"{}\"", path.display()))
}
