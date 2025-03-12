//! # cargo-caw-publish (checksum aware wrapper publish)
//! A thin wrapper around `cargo publish` that verifies if a crate is
//! publishable taking on account both version string and checksum.
//!
//!
//! ## Install
//!
//! `cargo install cargo-caw-publish`
//!
//!
//! ## Usage
//!
//! - verify the Cargo.toml file at the root of the current folder.
//!
//! ```bash
//! $ cargo caw-publish
//! ```
//!
//! - verify the Cargo.toml file at a path `<package_name>/Cargo.toml`
//!
//! ```bash
//! $ cargo caw-publish <package_name>
//! ```
//!
//! - pass extra arguments for the "cargo package" phase
//!
//! ```bash
//! $ cargo caw-publish --package-args="--allow-dirty --keep-going"
//! ```
//!
//! - pass extra arguments for the "cargo publish" phase
//!
//! ```bash
//! $ cargo caw-publish --publish-args="--all-features --keep-going"
//! ```
//!
//! ## Outputs
//!
//! Let's say our latest release was the version 1.2.3. So:
//!
//! - when you try to re-run publish with the same version we will return
//! (exit code 0):
//!
//! ```
//! the version <ver_string> is already published at the remote registry. nothing
//! to do here.
//! ```
//!
//! - when you change some code and forgets to update the version string we will
//! return (exit code 1):
//!
//! ```
//! the version <ver_string> is already published at the remote registry but your
//! local .crate checksum differs from the one on the remote:
//! local : <hash-a>
//! remote: <hash-b>
//! hint: maybe you forgot to update the version at <ver_string>!?
//! ```

use std::{
    path::{Path, PathBuf},
    process,
};

use clap::Parser;

use log::{debug, error, info};
use serde::Deserialize;
use sha256::try_digest;

use cargo_caw_publish::exec_cargo_package;
use cargo_caw_publish::exec_cargo_publish;
use cargo_caw_publish::exec_cargo_version;
use cargo_caw_publish::get_crate_data as get_cratesio_data;
use cargo_caw_publish::CargoToml;
use cargo_caw_publish::{init_logger, CargoWrapper};

#[derive(Debug, Deserialize, Clone)]
struct CrateVersion {
    checksum: String,
    num: String,
}

#[derive(Debug, Deserialize, Clone)]
struct CratesResponse {
    versions: Vec<CrateVersion>,
}

fn get_sha256_from_crate_file(crate_file_path: &Path) -> String {
    try_digest(crate_file_path).unwrap()
}

fn main() {
    let cli = match CargoWrapper::parse() {
        CargoWrapper::CawPublish(c) => c,
    };
    // let cli = Cli::parse();

    init_logger(cli.verbose);

    let crate_cargo_toml_path: PathBuf = match cli.package {
        Some(s) => Path::new(&format!("{}/Cargo.toml", s)).to_path_buf(),
        None => Path::new("Cargo.toml").to_path_buf(),
    };

    // Recovering Data from Cargo.toml
    // =========================================================================
    let cargo_toml = match CargoToml::from(&crate_cargo_toml_path) {
        Ok(m) => m,
        Err(e) => {
            error!("err while loading crate's toml: {}", e);
            process::exit(1);
        }
    };
    let crate_ver = cargo_toml.package.version;
    let crate_name = cargo_toml.package.name;

    // Recalculating Checksum
    // =========================================================================
    match exec_cargo_package(Some(&crate_name), cli.package_args) {
        Ok(s) => {
            if s.len() > 0 {
                info!("exec cargo package: {}", s)
            }
        }
        Err(e) => {
            error!("exec cargo package: {}", e);
            process::exit(1);
        }
    };
    let crate_file_path = PathBuf::from(format!(
        "./target/package/{}-{}.crate",
        crate_name, crate_ver
    ));
    if !crate_file_path.is_file() {
        error!("crate file does not exists: {}", crate_file_path.display());
        process::exit(1);
    }
    let crate_checksum = get_sha256_from_crate_file(crate_file_path.as_path());

    // Fetching data from Remote Registry
    // =========================================================================
    let cargo_ver = match exec_cargo_version() {
        Ok(s) => {
            let trimmed = s.trim().to_string();
            debug!("exec cargo version: {}", trimmed);
            trimmed
        }
        Err(e) => {
            let fallback_ver = "cargo 1.82.0 (8f40fc59f 2024-08-21)";
            error!("exec cargo version: {}", e);
            error!("exec cargo version: falling back to {}", fallback_ver);
            String::from(fallback_ver)
        }
    };
    let resp = get_cratesio_data(&crate_name, &cargo_ver);
    // TODO make this less nested?
    match resp {
        Ok(r) => {
            if r.status() == 404 {
                // Crate does NOT have an entry yet at the registry
                match exec_cargo_publish(Some(&crate_name), cli.publish_args) {
                    Ok(s) => {
                        if s.len() > 0 {
                            info!("exec cargo publish: {}", s)
                        }
                    }
                    Err(e) => {
                        error!("exec cargo publish: {}", e);
                        process::exit(1);
                    }
                };
                return;
            }
            if r.status() != 200 {
                // TODO recover HTTP Response body and add to the error message
                error!("remote registry err: {} - ", r.status());
                process::exit(1);
            }
            match r.json::<CratesResponse>() {
                Ok(api_resp) => {
                    for version in api_resp.versions {
                        if version.num == crate_ver {
                            if version.checksum == crate_checksum {
                                // TODO add a flag to make this scenario return
                                // 0
                                info!("the version {} is already published at the remote registry. nothing to do here.", version.num);
                                return;
                            } else {
                                error!(
                                    "the version {} is already published at the remote registry but your local .crate checksum differs from the one on the remote:\n\
                                    local : {}\n\
                                    remote: {}\n\
                                    hint: maybe you forgot to update the version at {}!?",
                                    version.num,
                                    crate_checksum,
                                    version.checksum,
                                    crate_cargo_toml_path.display(),
                                );
                                process::exit(1);
                            }
                        }
                    }
                    match exec_cargo_publish(Some(&crate_name), cli.publish_args) {
                        Ok(s) => {
                            if s.len() > 0 {
                                info!("exec cargo publish: {}", s)
                            }
                        }
                        Err(e) => {
                            error!("exec cargo publish: {}", e);
                            process::exit(1);
                        }
                    };
                    return;
                }
                Err(e) => {
                    error!("err parsing resp from remote registry: {}", e);
                    process::exit(1);
                }
            };
        }
        Err(e) => {
            error!("err while attempting to connect to remote registry: {}", e);
            process::exit(1);
        }
    };
}
