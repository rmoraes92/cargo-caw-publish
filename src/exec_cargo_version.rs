use std::process::Command;
use std::process::Output;

use anyhow::{bail, Result};
use log::debug;

/// executes "cargo version" command line and returns the output:
/// cargo 1.83.0 (5ffbef321 2024-10-29)
pub fn exec_cargo_version() -> Result<String> {
    let cmd = "cargo version";
    debug!("exec cargo version: {}", cmd);
    let output: Output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("failed to execute process")
    };
    if output.status.success() {
        return Ok(String::from_utf8(output.stdout)?);
    } else {
        bail!(String::from_utf8(output.stderr)?);
    }
}
