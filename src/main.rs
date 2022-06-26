use std::env;
use std::process;
use git_date_modifier::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = git_date_modifier::run(&config.directory) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
