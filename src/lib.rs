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
    set_new_path(&directory)?;
    let output = Command::new("git").arg("rebase").arg("-i").arg("a3879b5").output().expect("Error when launching command");
    print!("{}", String::from_utf8(output.stderr)?);
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

fn set_new_path(directory: &str) -> Result<(), &'static str> {
    let new_path = Path::new(&directory);
    std::env::set_current_dir(new_path).expect("Couldn't set path");
    Ok(())
}

fn get_path() -> PathBuf {
    std::env::current_dir().expect("Error while getting path")
}