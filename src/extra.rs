use std::fs::{File};
use std::io::Write;
use std::path::{Path};
use std::process::{exit};

pub fn is_git_repo(repo: &str) -> bool {
    if Path::new(format_args!("{}{}", repo, "/.git/").to_string().as_str()).exists() {
        return true
    } else { return false }
}

pub fn ignore_f(repo: &str, file: &str) {
    if is_git_repo(repo) != true {
        println!("{} is not a git repository", repo);
        exit(0);
    }
    let mut ignore_file = File::open(format_args!("{}{}", repo, "/.gitignore").to_string().as_str()).unwrap();
    ignore_file.write_fmt(format_args!("{}{}", file, " \n"));
}
pub fn ignore_d(repo: &str, dir: &str) {
    if is_git_repo(repo) != true {
        println!("{} is not a git repository", repo);
        exit(0);
    }
    let mut ignore_file = File::open(format_args!("{}{}", repo, "/.gitignore").to_string().as_str()).unwrap();
    ignore_file.write_fmt(format_args!("{}{}", dir, "/\n"));
}