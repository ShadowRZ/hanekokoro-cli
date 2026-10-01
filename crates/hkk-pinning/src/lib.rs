use std::{collections::BTreeMap, path::Path};

use thiserror::Error;

pub mod flake;

/// A Pinning method.
pub enum PinningMethod {
    /// Nix flake.
    Flake,
}

pub enum Codeforge {
    GitHub,
    GitLab,
    Sourcehut,
    Forgejo,
}

/// Original reference that can be resolved by input pinning.
pub enum OriginalRef {
    Tarball {
        url: String,
    },
    Codeforge {
        forge: Codeforge,
        owner: String,
        repo: String,
        git_ref: Option<String>,
        domain: Option<String>,
    },
    Git {
        /// URL of the Git repository.
        url: String,
        /// The Git ref that should be fetched by Nix.
        git_ref: String,
        /// Whether to fetch submodules.
        submodules: bool,
        /// Whether to fetch Git LFS content.
        lfs: bool,
    },
}

/// A resolved reference that can be turned into [`FetchableRef`].
pub enum ResolvedRef {
    Tarball {
        url: String,
        /// Hash of the entire content when unpacked.
        ///
        /// It might be specified in SRI, SHA-256 hex, or "nix32" format.
        hash: String,
    },
    Codeforge {
        forge: Codeforge,
        owner: String,
        repo: String,
        git_ref: String,
        domain: Option<String>,
        /// Hash of the entire content when unpacked.
        ///
        /// It might be specified in SRI, SHA-256 hex, or "nix32" format.
        hash: String,
    },
    Git {
        /// URL of the Git repository.
        url: String,
        /// The Git ref that should be fetched by Nix.
        git_ref: String,
        /// Whether to fetch submodules.
        submodules: bool,
        /// Whether to fetch Git LFS content.
        lfs: bool,
    },
}

/// A fetchable reference that can be fetched in Nix.
pub enum FetchableRef {
    /// A single fixed content file.
    ///
    /// In Nix, it is fetched with `builtins.fetchurl`.
    File {
        /// URL of the file.
        url: String,
        /// Hash of the file.
        ///
        /// It might be specified in SRI, SHA-256 hex, or "nix32" format.
        hash: String,
    },
    /// An tarball (or other unpackable format) that resolves to a directory of
    /// files.
    ///
    /// In Nix, it is fetched with `builtins.fetchTarball`, with derivation name
    /// of "source".
    Tarball {
        /// URL of the unpackable content.
        url: String,
        /// Hash of the entire content when unpacked.
        ///
        /// It might be specified in SRI, SHA-256 hex, or "nix32" format.
        hash: String,
    },
    /// A Git repository.
    ///
    /// In Nix, it is fetched with `builtins.fetchGit`.
    Git {
        /// URL of the Git repository.
        url: String,
        /// The Git ref that should be fetched by Nix.
        git_ref: String,
        /// Whether to fetch submodules.
        submodules: bool,
        /// Whether to fetch Git LFS content.
        lfs: bool,
    },
}

/// All pinned references of the root.
pub struct PinnedRefs {
    pub inputs: BTreeMap<String, (OriginalRef, ResolvedRef)>,
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("IO Error")]
    Io(
        #[from]
        #[source]
        std::io::Error,
    ),
    #[error("Parse failure while reading all pinned inputs")]
    Parse(
        #[from]
        #[source]
        serde_json::Error,
    ),
}

pub fn pinned_refs<P: AsRef<Path>>(root: P) -> Result<Option<PinnedRefs>, Error> {
    let mut flake_lock = root.as_ref().to_path_buf();
    flake_lock.push("flake.lock");

    if std::fs::exists(&flake_lock)? {
        use crate::flake::FlakeLock;

        let data = std::fs::read(flake_lock)?;
        let parsed: FlakeLock = serde_json::from_slice(&data)?;
        return Ok(Some(parsed.into()));
    }

    Ok(None)
}

pub fn pinning_type<P: AsRef<Path>>(root: P) -> Option<PinningMethod> {
    let mut flake_lock = root.as_ref().to_path_buf();
    flake_lock.push("flake.lock");
    if flake_lock.exists() {
        return Some(PinningMethod::Flake);
    }

    return None;
}
