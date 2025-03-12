use clap::Parser;

/// Mandatory Parse to properly handle "cargo" command
#[derive(Parser)]
#[command(version, about, long_about = None, name = "cargo", bin_name = "cargo")]
pub enum CargoWrapper {
    CawPublish(Cli),
}

#[derive(Parser)]
#[command(version, about, long_about = None, name = "caw-publish", bin_name = "caw-publish")]
pub struct Cli {
    /// name of the package/crate to be published
    pub package: Option<String>,

    /// list of args for "cargo package" command
    #[arg(long)]
    pub package_args: Option<String>,

    /// list of args for "cargo publish" command
    #[arg(long)]
    pub publish_args: Option<String>,

    #[arg(short, long)]
    pub verbose: bool,
}
