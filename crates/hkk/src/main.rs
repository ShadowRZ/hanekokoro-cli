mod cli;

use clap::Parser;

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
struct Cli {}

fn main() {
    let args = Cli::parse();
}
