//! # cargo-is-version-published
//! A thin wrapper around `cargo publish` that verifies if a crate is
//! publishable taking on account both version string and checksum.
//! 
//! 
//! ## Usage
//! 
//! ```bash
//! $ cargo is-version-published
//! ```
//! 
//! or
//! 
//! ```bash
//! $ cargo is-version-published <package_name>
//! ```

// TODO find a better name to this crate

use std::{path::{Path, PathBuf}, process};

use clap::Parser;

use serde::Deserialize;
use sha256::try_digest;

use cargo_is_version_published::Cli;
use cargo_is_version_published::CargoToml;
use cargo_is_version_published::exec_cargo_version;
use cargo_is_version_published::exec_cargo_package;
use cargo_is_version_published::exec_cargo_publish;
use cargo_is_version_published::get_crate_data as get_cratesio_data;

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
    let cli = Cli::parse();

    let crate_cargo_toml_path: PathBuf = match cli.package {
        Some(s) => Path::new(&format!("{}/Cargo.toml", s)).to_path_buf(),
        None => Path::new("Cargo.toml").to_path_buf(),
    };

    // Recovering Data from Cargo.toml
    // =========================================================================
    let cargo_toml = match CargoToml::from(&crate_cargo_toml_path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("err while loading crate's toml: {}", e);
            process::exit(1);
        },
    };
    let crate_ver = cargo_toml.package.version;
    let crate_name = cargo_toml.package.name;

    // Recalculating Checksum
    // =========================================================================
    match exec_cargo_package(Some(&crate_name), cli.package_args) {
        Ok(s) => if s.len() > 0 {
            println!("exec cargo package: {}", s)
        },
        Err(e) => {
            eprintln!("exec cargo package: {}", e);
            process::exit(1);
        },
    };
    let crate_file_path = PathBuf::from(format!(
        "./target/package/{}-{}.crate", crate_name, crate_ver));
    if !crate_file_path.is_file() {
        eprintln!("crate file does not exists: {}", crate_file_path.display());
        process::exit(1);
    }
    let crate_checksum = get_sha256_from_crate_file(
        crate_file_path.as_path());

    // Fetching data from Remote Registry
    // =========================================================================
    let cargo_ver = match exec_cargo_version() {
        Ok(s) => {
            let trimmed = s.trim().to_string();
            println!("exec cargo version: {}", trimmed);
            trimmed
        },
        Err(e) => {
            let fallback_ver = "cargo 1.82.0 (8f40fc59f 2024-08-21)";
            eprintln!("exec cargo version: {}", e);
            eprintln!("exec cargo version: falling back to {}", fallback_ver);
            String::from(fallback_ver)
        },
    }; 
    let resp = get_cratesio_data(&crate_name, &cargo_ver);
    // TODO make this less nested?
    match resp {
        Ok(r) => {
            if r.status() == 404 {
                // Crate does NOT have an entry yet at the registry
                match exec_cargo_publish(Some(&crate_name), cli.publish_args) {
                    Ok(s) => if s.len() > 0 {
                        println!("exec cargo publish: {}", s)
                    },
                    Err(e) => {
                        eprintln!("exec cargo publish: {}", e);
                        process::exit(1);
                    },
                };
                return
            }
            if r.status() != 200{
                // TODO recover HTTP Response body and add to the error message
                eprintln!("remote registry err: {} - ", r.status());
                process::exit(1);
            }
            match r.json::<CratesResponse>() {
                Ok(api_resp) => {
                    for version in api_resp.versions {
                        if version.num == crate_ver {
                            if version.checksum == crate_checksum {
                                // TODO add a flag to make this scenario return
                                // 0
                                eprintln!("the version {} is already published at the remote registry. nothing to do here.", version.num);
                                return
                            } else {
                                eprintln!(
                                    "the version {} is already published at \
                                    the remote registry but your local .crate \
                                    checksum differs from the one on the remote:\n\
                                    local : {}\n\
                                    remote: {}\n\
                                    hint: maybe you forgot to update the version \
                                    at {}!?",
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
                        Ok(s) => if s.len() > 0 {
                            println!("exec cargo publish: {}", s)
                        },
                        Err(e) => {
                            eprintln!("exec cargo publish: {}", e);
                            process::exit(1);
                        },
                    };
                    return
                },
                Err(e) => {
                    eprintln!("err parsing resp from remote registry: {}", e);
                    process::exit(1);
                },
            };
        },
        Err(e) => {
            eprintln!("err while attempting to connect to remote registry: {}", e);
            process::exit(1);
        },
    };
}
