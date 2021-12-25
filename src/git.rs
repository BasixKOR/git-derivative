use std::io::Result as IoResult;
use std::process::Command;

pub fn get_changed_files(old_rev: &str, new_rev: &str) -> IoResult<Vec<String>> {
    Ok(Command::new("git")
        .args(&["diff", "--name-only", old_rev, new_rev])
        .output()?
        .stdout
        .split(|b| *b == b'\n')
        .map(|b| String::from_utf8_lossy(b).into_owned())
        .collect())
}
