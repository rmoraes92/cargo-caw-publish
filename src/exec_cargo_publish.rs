use std::process::Command;
use std::process::Output;

use anyhow::{Result, bail};
use log::debug;
use string_from::Str;

pub fn exec_cargo_publish(package: Option<&str>, args: Option<String>) -> Result<String>{
    let cargo_cmd: String = match package {
        Some(p) => format!("cargo publish -p {}", p),
        None => Str!("cargo publish"),
    };
    let cargo_cmd: String = match args {
        Some(a) => format!("{} {}", cargo_cmd, a),
        _ => cargo_cmd,
    };
    debug!("exec cargo publish: {}", &cargo_cmd);
    let output: Output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &cargo_cmd])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&cargo_cmd)
            .output()
            .expect("failed to execute process")
    };
    if output.status.success() {
        return Ok(String::from_utf8(output.stdout)?);
    } else {
        bail!(String::from_utf8(output.stderr)?);
    }
}
