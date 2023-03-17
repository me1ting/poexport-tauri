use std::{fs, path::PathBuf};

use anyhow::{bail, Context, Result};
use serde::{de::DeserializeOwned, Serialize};

/// read yaml data as struct T
pub fn read_yaml<T: DeserializeOwned>(path: &PathBuf) -> Result<T> {
    if !path.exists() {
        bail!("file not found \"{}\"", path.display());
    }
    let yaml_str = fs::read_to_string(&path)
        .with_context(|| format!("failed to read the file \"{}\"", path.display()))?;

    serde_yaml::from_str::<T>(&yaml_str).with_context(|| {
        format!(
            "failed to read the file with yaml format \"{}\"",
            path.display()
        )
    })
}

pub fn write_yaml<T: Serialize>(path: &PathBuf, data: &T) -> Result<()> {
    let yaml_str = serde_yaml::to_string(data).context("failed to serialize data")?;
    fs::write(path, yaml_str)
        .with_context(|| format!("failed to write the file \"{}\"", path.display()))
}
