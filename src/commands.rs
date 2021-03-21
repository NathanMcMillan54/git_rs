use crate::{is_git_repo};
use std::env::{set_current_dir};
use std::process::{Command, exit};

// Main git commands
pub struct GitCommands;

impl GitCommands {
    pub fn clone(website: &str, author: &str, repo: &str) {
        Command::new("git")
            .arg("clone")
            .arg(format_args!("{}{}{}", website, author, repo).to_string().as_str())
            .spawn();
    }

    pub fn checkout_b(repo: &str, branch: &str) {
        if is_git_repo(repo) != true {
            println!("{} is not a git repository", repo);
            exit(0);
        }
        Command::new("git")
            .arg("checkout")
            .arg(branch)
            .spawn();
    }

    pub fn checkout_t(repo: &str, tag: &str) {
        if is_git_repo(repo) != true {
            println!("{} is not a git repository", repo);
            exit(0);
        }
        Command::new("git")
            .arg("checkout")
            .arg(tag)
            .spawn();
    }

    pub fn checkout_nb(repo: &str, branch: &str) {
        if is_git_repo(repo) != true {
            println!("{} is not a git repository", repo);
            exit(0);
        }
        Command::new("git")
            .arg("checkout")
            .arg("-b")
            .arg(repo)
            .spawn();
    }

    pub fn add(&mut self) {
        Command::new("git")
            .arg("add")
            .arg(".")
            .spawn();
    }

    pub fn commit(&mut self message: &str) {
        Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(message)
            .spawn();
    }

    pub fn push(&mut self, message: &str) {
        self.add();
        self.commit(message);
        Command::new("git")
            .arg("push")
            .spawn();
    }
}
