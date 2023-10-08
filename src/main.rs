use colored::*;
use inquire::Select;
use serde::Deserialize;
use std::fs;
use std::io;
use std::process::{Command, Stdio};
mod render_config;

#[derive(Deserialize)]
struct Config {
    prefixes: Vec<String>,
}

impl Config {
    fn from_file(file_path: &str) -> Config {
        let config_str = fs::read_to_string(file_path).expect("Failed to read config file");
        serde_json::from_str(&config_str).expect("Failed to parse config file")
    }
}

fn select_prefix(prefixes: Vec<String>) -> String {
    let selection = Select::new("Select git comment prefix", prefixes.clone()).prompt();

    match selection {
        Ok(prefix) => {
            let index = prefixes.iter().position(|p| p == &prefix).unwrap_or(0);
            match index {
                0 => println!("{}", prefix.blue()),
                1 => println!("{}", prefix.magenta()),
                2 => println!("{}", prefix.yellow()),
                3 => println!("{}", prefix.cyan()),
                4 => println!("{}", prefix.red()),
                5 => println!("{}", prefix.green()),
                6 => println!("{}", prefix.purple()),
                7 => println!("{}", prefix.white()),
                _ => println!("{}", prefix),
            }

            prefix.to_string()
        }
        Err(_) => {
            println!("{}", "No prefix selected!".red());
            std::process::exit(1);
        }
    }
}

fn comment() -> (String, String) {
    let mut title = String::new();
    println!("{}", "Write your comment:".white().bold());
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");

    let mut content = String::new();
    println!("{}", "Write your description:".white().bold());
    io::stdin()
        .read_line(&mut content)
        .expect("Failed to read line");

    (title.trim().to_string(), content.trim().to_string())
}

fn handle_git_commit(prefix: &str, title: &str, content: &str) {
    let commit_message = format!("{} {}\n\n{}", prefix, title, content);
    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&commit_message)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute git commit");

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout).white());
        println!("{}", "Commit successful!".green().bold());
    } else {
        println!("{}", String::from_utf8_lossy(&output.stderr).white());
        println!("{}", "Commit failed!".red().bold().italic());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    render_config::setup_inquire()?;
    let config = Config::from_file("./config.json");
    let prefix = select_prefix(config.prefixes);
    let (title, content) = comment();
    handle_git_commit(&prefix, &title, &content);

    Ok(())
}
