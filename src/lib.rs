use std::error::Error;
use std::process::{Command};
use std::fs;
use std::path::{Path, PathBuf};
use chrono::{self, DateTime, Local, TimeZone, NaiveDateTime};

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
    let commits = get_all_commits_and_dates(&directory)?;
    let first_commit = commits.first().unwrap();
    start_rebase(&directory, first_commit.hash.as_str(), &commits)?;
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

fn start_rebase(directory: &str, oldest_commit: &str, commits: &Vec<CommitAndDate>) -> Result<(), &'static str> {
    let initial_path = get_path();
    let initial_path = initial_path.as_path();

    set_new_path(&directory)?;

    Command::new("git").arg("rebase").arg("-i").arg("--root").output().expect("Error when launching command");
    let mut i = 0;
    loop {
        let output = Command::new("git").arg("status").output().expect("Error when launching command");
        let command_output = String::from_utf8(output.stdout).unwrap();
        let lines = command_output.split("\n").collect::<Vec<&str>>();
        if 2+i >= lines.len(){
            break;
        }
        i += 1;
        
        loop {
            let mut buffer = String::new();
            print!("Enter date in format %Y-%m-%d %H:%M:%S: ");
            let stdin = std::io::stdin(); // We get `Stdin` here.
            let res = stdin.read_line(&mut buffer);
            if let Err(e) = res {
                continue;
            }
            let from = NaiveDateTime::parse_from_str(&buffer.trim(), "%Y-%m-%d %H:%M:%S");
            if let Err(e) = from {
                eprintln!("Format is %Y-%m-%d %H:%M:%S");
                continue;
            }
            let date_time = Local.from_local_datetime(&from.unwrap()).unwrap();
            std::env::set_var("GIT_COMMITTER_DATE", date_time.format("%c").to_string());
            Command::new("git").args(["commit", "--amend", "--no-edit", "--date", date_time.format("%c").to_string().as_str()]).output().expect("Error when launching command");
            break;
        }
        
        Command::new("git").args(["rebase", "--continue"]).output().expect("Error when launching command");
    }
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

struct CommitAndDate {
    hash: String,
    date: DateTime<chrono::Local>,
}

impl CommitAndDate {
    fn new(hash: &str, date: &str) -> Result<CommitAndDate, &'static str> {
        let from: NaiveDateTime = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S").unwrap();
        let date_time = Local.from_local_datetime(&from).unwrap();

        return Ok(CommitAndDate {
            hash: hash.to_string().clone(),
            date: date_time,
        });
    }
}

fn get_all_commits_and_dates(directory: &str) -> Result<Vec<CommitAndDate>, &'static str> {
    let initial_path = get_path();
    let initial_path = initial_path.as_path();

    set_new_path(&directory)?;

    let mut result: Vec<CommitAndDate> = Vec::new();
    let output = Command::new("git").args(["log", "--oneline", "--pretty=format:\"%h %cd\"", "--date=format:'%Y-%m-%d %H:%M:%S'"]).output().expect("Error when launching command");
    let commits = String::from_utf8(output.stdout).expect("Couldn't launch number of commits command");
    let commits = commits.trim();
    for commit in commits.split("\n") {
        let commit = commit.replace("\"", "");
        let hash = &commit[..7];
        let date = &commit[7..];
        let whatever = date.clone().trim().replace("'", "");
        let date = whatever.as_str();
        println!("{} {}", hash, date);
        result.push(CommitAndDate::new(hash, date)?);

    }
    set_new_path(initial_path.as_os_str().to_str().unwrap())?;
    result.reverse();
    Ok(result)
}