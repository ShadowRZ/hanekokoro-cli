use std::{path::PathBuf, process::Command};

pub fn root_dir() -> Result<PathBuf, std::io::Error> {
    let git_cmd = Command::new("git").arg("rev-parse").arg("--show-toplevel").output()?;
    if git_cmd.status.success() {
        let path = String::from_utf8(git_cmd.stdout).unwrap();
        Ok(path.into())
    } else {
        std::env::current_dir()
    }
}
