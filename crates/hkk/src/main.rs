mod cli;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    display_name = "Hanekokoro CLI",
    version = clap::crate_version!(),
    about = cli::about(),
    arg_required_else_help = true,
    styles = cli::styles(),
    help_template = "
{before-help}{name} {version}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(subcommand, name = "nixos")]
    /// NixOS related functions
    ///
    /// Implements some functions that mostly built around, or with the idea of
    /// nixos-rebuild in mind.
    NixOS(hkk_nixos::NixOSCommand),
}

fn main() {
    let args = Cli::parse();
}
