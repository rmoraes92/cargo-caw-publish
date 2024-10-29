//! # cargo-is-version-published
//! Check if a Cargo.toml version was published without panic.
//!
//! ```bash
//! $ cargo is-version-published Cargo.toml  # <yes|no>
//! ```
//! 

use std::{env, path::Path, process};
use cfo::read_file;
use toml::Table;
use reqwest::{
    Method,
    blocking::Client as RwClient
};
use anyhow::Result;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct Version {
    // checksum: String,
    num: String,
}

#[derive(Debug, Deserialize, Clone)]
struct CratesResponse {
    // _crate: Crate,
    // meta: Meta,
    versions: Vec<Version>,
}

fn load_crate_toml(toml_path: &Path) -> Result<Table> {
    let crate_toml_str = read_file(toml_path)?;
    let crate_toml = crate_toml_str.parse::<Table>()?;
    return Ok(crate_toml);
}

fn get_crates_version(crate_toml: &Table) -> Option<String> {
    Some(crate_toml.get("package")?.as_table()?.get("version")?.as_str()?.to_string())
}

fn get_crates_name(crate_toml: &Table) -> Option<String> {
    Some(crate_toml.get("package")?.as_table()?.get("name")?.as_str()?.to_string())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("missing Cargo.toml path! Usage:");
        eprintln!("\tcargo is-version-published Cargo.toml");
        eprintln!("\tcargo is-version-published workspace_proj_1/Cargo.toml");
        process::exit(1);
    }
    let crate_toml_path = Path::new(&args[1]);
    let crate_toml = match load_crate_toml(&crate_toml_path) {
        Ok(m) => m,
        Err(e) => {
            eprint!("err while loading crate's toml: {}", e);
            process::exit(1);
        },
    };
    let crate_toml_ver = match get_crates_version(&crate_toml) {
        Some(v) => v,
        None => {
            eprintln!("could not recover crate's version from toml.");
            process::exit(1);
        },
    };
    let crate_name = match get_crates_name(&crate_toml) {
        Some(v) => v,
        None => {
            eprintln!("could not recover crate's name from toml.");
            process::exit(1);
        },
    };
    let host = "https://crates.io";
    let endpoint = "/api/v1/crates";
    let url = format!("{}{}/{}", host, endpoint, crate_name);
    let cargo_ver = "cargo 1.82.0 (8f40fc59f 2024-08-21)"; // TODO make this dynamic?
    let req_builder = RwClient::new()
        .request(Method::GET, url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("User-Agent", cargo_ver);
    let resp = req_builder.send();
    match resp {
        Ok(r) => {
            if r.status() == 404 {
                // presuming the crate was not published at all
                print!("no");
                return;
            }
            if r.status() != 200{
                eprintln!("crates.io api err: {}", r.status());
                process::exit(1);
            }
            match r.json::<CratesResponse>() {
                Ok(api_resp) => {
                    for version in api_resp.versions {
                        if version.num == crate_toml_ver {
                            print!("yes");
                            return;
                        }
                    }
                    print!("no");
                },
                Err(e) => {
                    eprint!("err parsing resp from crates.io: {}", e);
                    process::exit(1);
                },
            };
        },
        Err(e) => {
            eprint!("err on http request to crates.io: {}", e);
            process::exit(1);
        },
    };
}
