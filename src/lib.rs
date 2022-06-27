use std::error::Error;
use std::process::{Command};
use std::fs;
use std::path::{Path, PathBuf};

pub struct Config {
    pub directory: String
}
    
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }
        return Ok(Config {
            directory: args[1].clone(),
        });
    }
}

pub fn run(directory: &str) -> Result<(), Box<dyn Error>> {
    println!("Should search trough {}", directory);
    is_git_installed()?;
    does_directory_exist(&directory)?;
    is_git_initialised(&directory)?;
    let initial_commit = get_oldest_commit(&directory)?;
    start_rebase(&directory, &initial_commit)?;
    Ok(())
}

fn is_git_installed() -> Result<(), &'static str> {
    let output = Command::new("git").arg("version").output().expect("Error when launching command");
    if !output.status.success() {
        return Err("Git is not installed");
    }
    Ok(())
}

fn does_directory_exist(directory: &str) -> Result<(), &'static str> {
    let path = Path::new(directory);
    fs::read_dir(path).expect("Couldn't open file");
    Ok(())
}

fn is_git_initialised(directory: &str) -> Result<(), &'static str> {
    let initial_path = get_path();
    let initial_path = initial_path.as_path();
    set_new_path(&directory)?;    

    let current_path = get_path();
    let current_path = current_path.as_path();

    let output = Command::new("git").arg("status").output().expect("Error when launching command");
    set_new_path(initial_path.as_os_str().to_str().unwrap())?;
    if !output.status.success(){
        eprintln!("{}", current_path.display());
        return Err("Not a git repository")
    }
    Ok(())
}

fn get_oldest_commit(directory: &str) -> Result<String, &'static str> {
    let initial_path = get_path();
    let initial_path = initial_path.as_path();

    set_new_path(&directory)?;

    let number_of_commits = Command::new("git").arg("rev-list").arg("--count").arg("HEAD").output().expect("Error when launching command");
    let number_of_commits = String::from_utf8(number_of_commits.stdout).expect("Couldn't launch number of commits command");
    let mut number_of_commits: i32 = number_of_commits.trim().parse().expect("Couldn't parse number of commits");
    number_of_commits -= 1;

    if number_of_commits <= 0 {
        return Err("No commits");
    }

    let oldest_commit = Command::new("git").arg("rev-parse").arg(format!("HEAD~{}", number_of_commits)).output().expect("Error when launching command");
    let oldest_commit = String::from_utf8(oldest_commit.stdout).expect("Couldn't launch number of commits command");
    let oldest_commit = oldest_commit.trim().to_string();
    set_new_path(initial_path.as_os_str().to_str().unwrap())?;

    Ok(oldest_commit)
}

fn start_rebase(directory: &str, oldest_commit: &str) -> Result<(), &'static str> {
    let initial_path = get_path();
    let initial_path = initial_path.as_path();
    set_new_path(&directory)?;

    let output = Command::new("git").arg("rebase").arg("-i").arg(oldest_commit).output().expect("Error when launching command");
    set_new_path(initial_path.as_os_str().to_str().unwrap())?;
    Ok(())
}

fn set_new_path(directory: &str) -> Result<(), &'static str> {
    let new_path = Path::new(&directory);
    std::env::set_current_dir(new_path).expect("Couldn't set path");
    Ok(())
}

fn get_path() -> PathBuf {
    std::env::current_dir().expect("Error while getting path")
}