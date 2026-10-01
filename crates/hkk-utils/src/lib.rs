use std::{path::PathBuf, process::Command};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RootDirError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Git worktree root is invalid UTF-8")]
    GitRootNotUtf8(
        #[from]
        #[source]
        std::string::FromUtf8Error,
    ),
}

pub fn root_dir() -> Result<PathBuf, RootDirError> {
    let git_cmd = Command::new("git").arg("rev-parse").arg("--show-toplevel").output()?;
    if git_cmd.status.success() {
        let path = String::from_utf8(git_cmd.stdout).map_err(RootDirError::GitRootNotUtf8)?;
        Ok(path.trim().into())
    } else {
        let cwd = std::env::current_dir()?;
        Ok(cwd)
    }
}
