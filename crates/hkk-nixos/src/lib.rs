mod build;

use clap::{Args, Subcommand};
use hkk_arguments::InstallableArgs;

#[derive(Debug, Args)]
pub struct NixOSArgs {
    #[command(subcommand)]
    command: NixOSCommand,
}

#[derive(Debug, Subcommand)]
pub enum NixOSCommand {
    /// Build a NixOS configuration
    Build(NixOSBuildArgs),
}

#[derive(Debug, Args)]
pub struct NixOSBuildArgs {
    #[command(flatten)]
    common: CommonBuildArgs,
}

#[derive(Debug, Args)]
pub struct CommonBuildArgs {
    #[command(flatten)]
    installable: InstallableArgs,
}

pub fn run(args: NixOSArgs) -> color_eyre::Result<()> {
    match args.command {
        NixOSCommand::Build(args) => self::build::build(args),
    }
}
