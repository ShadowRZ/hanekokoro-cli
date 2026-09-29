use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum NixOSCommand {
    /// Build a NixOS configuration
    Build,
}
