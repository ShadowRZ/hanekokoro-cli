use hkk_arguments::InstallableArgs;
use hkk_installable::AttrPath;

use crate::NixOSBuildArgs;

pub(super) fn build(args: NixOSBuildArgs) {
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
            let hostname = nix::unistd::gethostname().unwrap().into_string().unwrap();
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
            let attrpath = attrpath.render().unwrap();
            std::process::Command::new("nix")
                .args([
                    "--extra-experimental-features",
                    "nix-command flakes",
                    "build",
                    &format!("{flakeref}#{attrpath}"),
                ])
                .spawn()
                .unwrap();
        },
        _ => {},
    }
}
