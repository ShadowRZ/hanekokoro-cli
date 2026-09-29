use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttrPath(pub Vec<String>);

impl AttrPath {
    pub fn render(self) -> Result<String, RenderError> {
        let mut ret = String::new();

        for (idx, attr) in self.0.into_iter().enumerate() {
            if idx > 0 {
                ret.push('.');
            }

            if attr.contains('"') {
                return Err(RenderError(RenderErrorKind::Unrepresentable(attr)));
            }

            if attr.is_empty() || attr.contains('.') {
                ret += "\"";
                ret += &attr;
                ret += "\"";
            } else {
                ret += &attr;
            }
        }

        Ok(ret)
    }
}

/// A Nix installable representation.
///
/// This is based on [the implementation as part of a Nix experimental feature](https://nix.dev/manual/nix/2.35/command-ref/new-cli/nix.html#installables), but generalized for Nix2 / Nix3 CLI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Installable {
    /// A flake output attribute.
    Flake {
        /// A [flake reference](https://nix.dev/manual/nix/2.35/command-ref/new-cli/nix3-flake#flake-references).
        ///
        /// This is not parsed further, and should be passed as is.
        flakeref: String,
        /// Optional attrpath to eval.
        ///
        /// If empty, the exact attrpath to eval is determined by
        /// Nix itself.
        attrpath: AttrPath,
    },
    /// A store path.
    Store(String),
    /// A Nix file.
    File {
        /// Nix file or directory with `default.nix`.
        path: String,
        /// Optional attrpath to eval.
        ///
        /// If empty, assumes the [`path`][Self::File::path] itself
        /// produces a derivation.
        attrpath: AttrPath,
    },
    /// A Nix expression.
    Expression {
        /// Nix expression.
        expr: String,
        /// Optional attrpath to eval.
        ///
        /// If empty, assumes the [`expr`][Self::Expression::expr]
        /// itself produces a derivation.
        attrpath: AttrPath,
    },
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct ParseError(ParseErrorKind);

#[derive(Debug, Error)]
enum ParseErrorKind {
    #[error("attrpath has unclosed quote")]
    UnclosedQuote,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct RenderError(RenderErrorKind);

#[derive(Debug, Error)]
enum RenderErrorKind {
    #[error("attribute {0} contains a \", which is not representable in Nix.")]
    Unrepresentable(String),
}

impl FromStr for AttrPath {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = Vec::new();

        if s.is_empty() {
            return Ok(Self(components));
        }

        let mut component = String::new();
        let mut in_quote = false;

        for chr in s.chars() {
            match chr {
                '.' if !in_quote => components.push(std::mem::take(&mut component)),
                '"' => in_quote = !in_quote,
                chr => component.push(chr),
            }
        }

        if in_quote {
            return Err(ParseError(ParseErrorKind::UnclosedQuote));
        }

        components.push(component);

        Ok(Self(components))
    }
}

pub fn parse_flake_output(s: &str) -> Result<Installable, ParseError> {
    let (flakeref, attrpath) = s.split_once('#').unwrap_or((s, ""));

    if flakeref.starts_with("/nix/store/") {
        return Ok(Installable::Store(flakeref.to_string()));
    }

    Ok(Installable::Flake {
        flakeref: flakeref.to_string(),
        attrpath: attrpath.parse()?,
    })
}

#[cfg(test)]
mod test {
    use super::{AttrPath, Installable};

    #[test]
    fn attrpath_basic() {
        let attrpath = "packages.x86_64-linux.hello";
        let parsed: AttrPath = attrpath.parse().unwrap();
        assert_eq!(
            parsed,
            AttrPath(vec![
                "packages".to_string(),
                "x86_64-linux".to_string(),
                "hello".to_string()
            ])
        )
    }

    #[test]
    fn attrpath_empty() {
        let attrpath = "";
        let parsed: AttrPath = attrpath.parse().unwrap();
        assert_eq!(parsed, AttrPath(vec![]))
    }

    #[test]
    fn attrpath_quoted() {
        let attrpath = "packages.x86_64-linux.\"hello.world\"";
        let parsed: AttrPath = attrpath.parse().unwrap();
        assert_eq!(
            parsed,
            AttrPath(vec![
                "packages".to_string(),
                "x86_64-linux".to_string(),
                "hello.world".to_string()
            ])
        )
    }

    #[test]
    fn attrpath_backslash_is_literal() {
        let attrpath = "hello.a\\b";
        let parsed: AttrPath = attrpath.parse().unwrap();
        assert_eq!(parsed, AttrPath(vec!["hello".to_string(), "a\\b".to_string()]))
    }

    #[test]
    fn attrpath_invalid() {
        let attrpath = "hello.hello\\\"hello";
        assert!(attrpath.parse::<AttrPath>().is_err());
    }

    #[test]
    fn attrpath_to_string_basic() {
        let attrpath = AttrPath(vec![
            "packages".to_string(),
            "x86_64-linux".to_string(),
            "hello".to_string(),
        ]);

        assert_eq!(attrpath.render().unwrap(), "packages.x86_64-linux.hello");
    }

    #[test]
    fn attrpath_to_string_with_dots() {
        let attrpath = AttrPath(vec![
            "packages".to_string(),
            "x86_64-linux".to_string(),
            "hello.world".to_string(),
        ]);

        assert_eq!(attrpath.render().unwrap(), "packages.x86_64-linux.\"hello.world\"");
    }

    #[test]
    fn flake_installable_basic() {
        let flake_attr = "nixpkgs#hello";
        let parsed: Installable = super::parse_flake_output(flake_attr).unwrap();

        assert_eq!(
            parsed,
            Installable::Flake {
                flakeref: "nixpkgs".to_string(),
                attrpath: AttrPath(vec!["hello".to_string()])
            }
        );
    }

    #[test]
    fn flake_installable_without_attrpath() {
        let flake_attr = ".";
        let parsed: Installable = super::parse_flake_output(flake_attr).unwrap();

        assert_eq!(
            parsed,
            Installable::Flake {
                flakeref: ".".to_string(),
                attrpath: AttrPath(vec![])
            }
        );
    }
}
