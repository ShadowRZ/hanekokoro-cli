use std::collections::BTreeMap;

use serde::Deserialize;

use crate::PinnedRefs;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct FlakeLock {
    pub root: String,
    pub nodes: BTreeMap<String, FlakeNode>,
    pub version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path(pub Vec<String>);

impl<'de> Deserialize<'de> for Path {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum EitherPath {
            Single(String),
            Multiple(Vec<String>),
        }

        let either = EitherPath::deserialize(deserializer)?;

        Ok(match either {
            EitherPath::Single(path) => Self(vec![path]),
            EitherPath::Multiple(paths) => Self(paths),
        })
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct FlakeNode {
    pub inputs: BTreeMap<String, Path>,
    pub locked: Option<Locked>,
    pub original: Option<OriginalRef>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Locked {
    pub last_modified: u32,
    pub nar_hash: String,
    #[serde(flatten)]
    pub locked_ref: LockedRef,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum OriginalRef {
    #[serde(rename = "github")]
    GitHub {
        host: Option<String>,
        owner: String,
        repo: String,
        #[serde(rename = "ref")]
        git_ref: Option<String>,
        dir: Option<String>,
    },
    #[serde(rename = "gitlab")]
    GitLab {
        host: Option<String>,
        owner: String,
        repo: String,
        #[serde(rename = "ref")]
        git_ref: Option<String>,
        dir: Option<String>,
    },
    #[serde(rename = "sourcehut")]
    SourceHut {
        host: Option<String>,
        owner: String,
        repo: String,
        #[serde(rename = "ref")]
        git_ref: Option<String>,
        dir: Option<String>,
    },
    #[serde(rename = "tarball")]
    Tarball { url: String, dir: Option<String> },
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum LockedRef {
    #[serde(rename = "github")]
    GitHub {
        host: Option<String>,
        owner: String,
        repo: String,
        rev: String,
    },
    #[serde(rename = "gitlab")]
    GitLab {
        host: Option<String>,
        owner: String,
        repo: String,
        rev: String,
    },
    #[serde(rename = "sourcehut")]
    SourceHut {
        host: Option<String>,
        owner: String,
        repo: String,
        rev: String,
    },
    #[serde(rename = "tarball")]
    Tarball { url: String },
}

impl From<FlakeLock> for PinnedRefs {
    fn from(value: FlakeLock) -> Self {
        let mut inputs = BTreeMap::new();

        for (input, node) in value.nodes.into_iter() {
            use crate::Codeforge;

            let Some(flake_original) = node.original else {
                continue;
            };
            let Some(flake_locked) = node.locked else {
                continue;
            };

            let original_ref = match flake_original {
                OriginalRef::GitHub {
                    host,
                    owner,
                    repo,
                    git_ref,
                    ..
                } => crate::OriginalRef::Codeforge {
                    forge: Codeforge::GitHub,
                    owner,
                    repo,
                    git_ref: git_ref,
                    domain: host,
                },
                OriginalRef::GitLab {
                    host,
                    owner,
                    repo,
                    git_ref,
                    ..
                } => crate::OriginalRef::Codeforge {
                    forge: Codeforge::GitLab,
                    owner,
                    repo,
                    git_ref: git_ref,
                    domain: host,
                },
                OriginalRef::SourceHut {
                    host,
                    owner,
                    repo,
                    git_ref,
                    ..
                } => crate::OriginalRef::Codeforge {
                    forge: Codeforge::Sourcehut,
                    owner,
                    repo,
                    git_ref: git_ref,
                    domain: host,
                },
                OriginalRef::Tarball { url, .. } => crate::OriginalRef::Tarball { url },
            };

            let resolved_ref = {
                let Locked {
                    nar_hash, locked_ref, ..
                } = flake_locked;

                match locked_ref {
                    LockedRef::GitHub { host, owner, repo, rev } => crate::ResolvedRef::Codeforge {
                        forge: Codeforge::GitHub,
                        owner,
                        repo,
                        git_ref: rev,
                        domain: host,
                        hash: nar_hash,
                    },
                    LockedRef::GitLab { host, owner, repo, rev } => crate::ResolvedRef::Codeforge {
                        forge: Codeforge::GitHub,
                        owner,
                        repo,
                        git_ref: rev,
                        domain: host,
                        hash: nar_hash,
                    },
                    LockedRef::SourceHut { host, owner, repo, rev } => crate::ResolvedRef::Codeforge {
                        forge: Codeforge::GitHub,
                        owner,
                        repo,
                        git_ref: rev,
                        domain: host,
                        hash: nar_hash,
                    },
                    LockedRef::Tarball { url, .. } => crate::ResolvedRef::Tarball { url, hash: nar_hash },
                }
            };

            inputs.insert(input, (original_ref, resolved_ref));
        }

        Self { inputs }
    }
}

#[cfg(test)]
mod test {
    use super::{FlakeLock, Locked, LockedRef, OriginalRef};

    #[test]
    fn parse_flake_lock() {
        let lock = r#"
            {
              "nodes": {
                "root": {
                  "inputs": {
                    "treefmt-nix": "treefmt-nix"
                  }
                },
                "treefmt-nix": {
                  "inputs": {
                    "nixpkgs": []
                  },
                  "locked": {
                    "lastModified": 1772660329,
                    "narHash": "sha256-IjU1FxYqm+VDe5qIOxoW+pISBlGvVApRjiw/Y/ttJzY=",
                    "owner": "numtide",
                    "repo": "treefmt-nix",
                    "rev": "3710e0e1218041bbad640352a0440114b1e10428",
                    "type": "github"
                  },
                  "original": {
                    "owner": "numtide",
                    "repo": "treefmt-nix",
                    "type": "github"
                  }
                }
              },
              "root": "root",
              "version": 7
            }
            "#;
        let de = &mut serde_json::Deserializer::from_str(lock);
        let parsed: FlakeLock = serde_path_to_error::deserialize(de).unwrap();
        assert_eq!(parsed.root, "root");
        assert_eq!(
            parsed.nodes["treefmt-nix"].original,
            Some(OriginalRef::GitHub {
                host: None,
                owner: "numtide".to_string(),
                repo: "treefmt-nix".to_string(),
                git_ref: None,
                dir: None
            })
        );
        assert_eq!(
            parsed.nodes["treefmt-nix"].locked,
            Some(Locked {
                last_modified: 1772660329,
                nar_hash: "sha256-IjU1FxYqm+VDe5qIOxoW+pISBlGvVApRjiw/Y/ttJzY=".to_string(),
                locked_ref: LockedRef::GitHub {
                    host: None,
                    owner: "numtide".to_string(),
                    repo: "treefmt-nix".to_string(),
                    rev: "3710e0e1218041bbad640352a0440114b1e10428".to_string(),
                }
            })
        );
    }
}
