use hkk_arguments::InstallableArgs;
use hkk_installable::AttrPath;

use crate::NixOSBuildArgs;

pub(super) fn build(args: NixOSBuildArgs) -> color_eyre::Result<()> {
    use hkk_installable::Installable;

    let installable_args = args.common.installable;
    let installable = match installable_args {
        InstallableArgs::Specified(installable) => match installable {
            Installable::Flake { flakeref, attrpath } => {
                let mut new_attrpath = vec!["nixosConfigurations".to_string()];
                new_attrpath.extend(attrpath.0);
                new_attrpath.extend([
                    "config".to_string(),
                    "system".to_string(),
                    "build".to_string(),
                    "toplevel".to_string(),
                ]);
                Installable::Flake {
                    flakeref,
                    attrpath: AttrPath(new_attrpath),
                }
            },
            _ => installable,
        },
        InstallableArgs::Unspecified => {
            let hostname = nix::unistd::gethostname()?
                .into_string()
                .map_err(|_| color_eyre::eyre::eyre!("OS Hostname is not UTF-8"))?;
            Installable::Flake {
                flakeref: ".".to_string(),
                attrpath: AttrPath(vec![
                    "nixosConfigurations".to_string(),
                    hostname,
                    "config".to_string(),
                    "system".to_string(),
                    "build".to_string(),
                    "toplevel".to_string(),
                ]),
            }
        },
    };

    match installable {
        Installable::Flake { flakeref, attrpath } => {
            let attrpath = attrpath.render()?;
            let status = std::process::Command::new("nix")
                .args([
                    "--extra-experimental-features",
                    "nix-command flakes",
                    "build",
                    &format!("{flakeref}#{attrpath}"),
                ])
                .status()?;

            if !status.success() {
                use owo_colors::OwoColorize;

                match status.code() {
                    Some(code) => eprintln!(
                        "\n{} {}.",
                        "Nix command failed with exit code".red().bold(),
                        code.red().bold()
                    ),
                    None => eprintln!("\n{}.", "Nix command terminated with a signal.".red().bold()),
                };
            }
        },
        _ => {},
    };

    Ok(())
}
