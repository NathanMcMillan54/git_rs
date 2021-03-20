use std::path::{Path};

pub fn is_git_repo(repo: &str) -> bool {
    if Path::new(format_args!("{}{}", repo, "/.git/").to_string().as_str()).exists() {
        return true
    } else { return false }
}
