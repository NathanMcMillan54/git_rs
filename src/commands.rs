use crate::{is_git_repo};
use std::env::{set_current_dir};
use std::process::{Command, exit};

// Main git commands
pub struct Git;

impl Git {
    pub fn clone(website: &str, author: &str, repo: &str) {
        Command::new("git")
            .arg("clone")
            .arg(format_args!("{}{}{}", website, author, repo).to_string().as_str())
            .spawn();
        set_current_dir(repo);
    }

    pub fn checkoutb(repo: &str, branch: &str) {
        if is_git_repo(repo) != true {
            println!("{} is not a git repository", repo);
            exit(0);
        }
        set_current_dir(repo);
        Command::new("git")
            .arg("checkout")
            .arg(branch)
            .spawn();
    }

    pub fn checkoutt(repo: &str, tag: &str) {
        if is_git_repo(repo) != true {
            println!("{} is not a git repository", repo);
            exit(0);
        }
        set_current_dir(repo);
        Command::new("git")
            .arg("checkout")
            .arg(tag)
            .spawn();
    }
}
