use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
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
