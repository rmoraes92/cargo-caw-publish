use log::debug;
use std::path::Path;

use anyhow::Result;
use serde::Deserialize;
use toml::Table;
use cfo::read_file;

#[derive(Debug, Deserialize, Clone)]
pub struct CargoTomlPackage {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CargoToml {
    pub package: CargoTomlPackage
}

impl CargoToml {
    pub fn from(file_p: &Path) -> Result<Self> {
        debug!("reading crate file: {}", file_p.display());
        let crate_toml_str = read_file(file_p)?;
        let ret: Self = toml::from_str::<Self>(&crate_toml_str)?;
        return Ok(ret)
    }
}

pub fn load_crate_toml(toml_path: &Path) -> Result<Table> {
    let crate_toml_str = read_file(toml_path)?;
    let crate_toml = crate_toml_str.parse::<Table>()?;
    return Ok(crate_toml);
}

pub fn get_crates_version(crate_toml: &Table) -> Option<String> {
    Some(crate_toml.get("package")?.as_table()?.get("version")?.as_str()?.to_string())
}

pub fn get_crates_name(crate_toml: &Table) -> Option<String> {
    Some(crate_toml.get("package")?.as_table()?.get("name")?.as_str()?.to_string())
}
