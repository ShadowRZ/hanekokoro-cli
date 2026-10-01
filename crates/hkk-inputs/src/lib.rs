use owo_colors::OwoColorize as _;

pub fn run() -> color_eyre::Result<()> {
    let root = hkk_utils::root_dir()?;

    if let Some(pinned_refs) = hkk_pinning::pinned_refs(&root)? {
        use hkk_pinning::{Codeforge, ResolvedRef};
        use hkk_termlink::TerminalLink as _;

        println!("{}", "Inputs".bold());
        for (name, (_, resolved_ref)) in pinned_refs.inputs {
            print!("{}", "* ".bold());
            println!("{}", name.bold());

            let resolved_str = {
                match resolved_ref {
                    ResolvedRef::Tarball { url, .. } => url,
                    ResolvedRef::Codeforge {
                        forge,
                        owner,
                        repo,
                        git_ref,
                        domain,
                        ..
                    } => {
                        let forge_ref = match forge {
                            Codeforge::GitHub => "github",
                            Codeforge::GitLab => "gitlab",
                            Codeforge::Sourcehut => "sourcehut",
                            Codeforge::Forgejo => "forgejo",
                        };

                        let domain = domain.unwrap_or_else(|| {
                            match forge {
                                Codeforge::GitHub => "github.com",
                                Codeforge::GitLab => "gitlab.com",
                                Codeforge::Sourcehut => "git.sr.ht",
                                Codeforge::Forgejo => "codeberg.org",
                            }
                            .to_string()
                        });

                        let terminal_link = match forge {
                            Codeforge::GitHub | Codeforge::Forgejo => {
                                format!("https://{domain}/{owner}/{repo}/commit/{git_ref}")
                            },
                            Codeforge::GitLab => format!("https://{domain}/{owner}/{repo}/-/commit/{git_ref}"),
                            Codeforge::Sourcehut => todo!(),
                        };

                        format!("{forge_ref}:{owner}/{repo}/{git_ref}")
                            .terminal_link(&terminal_link)
                            .to_string()
                    },
                    ResolvedRef::Git { url, .. } => url,
                }
            };

            println!(" {} {}", "->".bold(), resolved_str)
        }
    } else {
        println!("{} {}.", "No pinning method detected in".bold(), root.display().bold());
    }

    Ok(())
}
