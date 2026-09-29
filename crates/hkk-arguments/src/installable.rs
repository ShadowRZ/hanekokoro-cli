use std::path::PathBuf;

use clap::{Arg, ArgAction, Args, FromArgMatches, ValueHint, error::ErrorKind};
use hkk_installable::Installable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallableArgs {
    Specified(Installable),
    Unspecified,
}

impl Args for InstallableArgs {
    fn augment_args(cmd: clap::Command) -> clap::Command {
        Self::augment_args_for_update(cmd)
    }

    fn augment_args_for_update(cmd: clap::Command) -> clap::Command {
        use owo_colors::OwoColorize;

        cmd.arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .action(ArgAction::Set)
                .hide(true),
        )
        .arg(
            Arg::new("expr")
                .short('e')
                .long("expr")
                .action(ArgAction::Set)
                .hide(true),
        )
        .arg(
            Arg::new("installable")
                .action(ArgAction::Set)
                .value_hint(ValueHint::AnyPath)
                .value_name("INSTALLABLE")
                .long_help(format!(
                    r#"
A installable that Nix can realize.

You can provide:

[FLAKEREF[#ATTRPATH]]
    Flake reference with an optional attribute path.

{}, {} <FILE> [ATTRPATH]
    Path to a Nix file with an optional attribute path.

{}, {} <EXPR> [ATTRPATH]
    Nix expression with an optional attribute path.

[PATH]
    Path or symlink to a /nix/store path"#,
                    "-f".cyan().bold(),
                    "--file".cyan().bold(),
                    "-e".cyan().bold(),
                    "--expr".cyan().bold()
                )),
        )
    }
}

impl FromArgMatches for InstallableArgs {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        let installable = matches.get_one::<String>("installable");
        let file = matches.get_one::<String>("file");
        let expr = matches.get_one::<String>("expr");

        if let Some(i) = installable {
            let canonical = std::fs::canonicalize(i);

            if let Ok(p) = canonical
                && p.starts_with("/nix/store")
            {
                return Ok(Self::Specified(Installable::Store(p)));
            }
        }

        if let Some(f) = file {
            let attrpath = f
                .parse()
                .map_err(|err| clap::Error::raw(ErrorKind::ValueValidation, format!("attribute path {err}")))?;
            return Ok(Self::Specified(Installable::File {
                path: PathBuf::from(f),
                attrpath,
            }));
        }

        if let Some(e) = expr {
            let attrpath = e
                .parse()
                .map_err(|err| clap::Error::raw(ErrorKind::ValueValidation, format!("attribute path {err}")))?;
            return Ok(Self::Specified(Installable::Expression {
                expr: e.clone(),
                attrpath,
            }));
        }

        if let Some(i) = installable {
            let (flakeref, attrpath) = hkk_installable::parse_flakeref(i)
                .map_err(|err| clap::Error::raw(ErrorKind::ValueValidation, format!("installable argument {err}")))?;
            return Ok(Self::Specified(Installable::Flake { flakeref, attrpath }));
        }

        Ok(Self::Unspecified)
    }

    fn update_from_arg_matches(
        &mut self,
        matches: &clap::ArgMatches,
    ) -> Result<(), clap::Error> {
        *self = Self::from_arg_matches(matches)?;
        Ok(())
    }
}
