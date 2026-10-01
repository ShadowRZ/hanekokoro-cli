pub fn run() -> color_eyre::Result<()> {
    let root = hkk_utils::root_dir()?;

    if let Some(pinned_refs) = hkk_pinning::pinned_refs(root)? {
        use hkk_pinning::{Codeforge, ResolvedRef};
        use hkk_termlink::TerminalLink as _;
        use owo_colors::OwoColorize;

        println!("{}", "Inputs".bold());
        for (name, (original_ref, resolved_ref)) in pinned_refs.inputs {
            print!("{}", "* ".bold());
            println!("{}", name.bold());

            let resolved_str = {
                match resolved_ref {
                    ResolvedRef::Tarball { url, hash } => url,
                    ResolvedRef::Codeforge {
                        forge,
                        owner,
                        repo,
                        git_ref,
                        domain,
                        hash,
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

                        format!("{forge_ref}:{owner}/{repo}/{git_ref}")
                            .terminal_link("")
                            .to_string()
                    },
                    ResolvedRef::Git {
                        url,
                        git_ref,
                        submodules,
                        lfs,
                    } => todo!(),
                };
            };
        }
    }

    Ok(())
}
