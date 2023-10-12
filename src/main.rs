extern crate colored;
extern crate dirs;
extern crate inquire;
extern crate serde;

use colored::*;
use inquire::{Select, Text};
use std::process::{Command, Stdio};
use config::Config;
use dirs::home_dir;

mod config;
mod render_config;

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
    let title = Text::new("Write your comment:")
        .with_help_message("Enter the title of your commit")
        .prompt()
        .unwrap_or_else(|_| {
            println!("{}", "No comment entered!".red());
            std::process::exit(1);
        });

    let content = Text::new("Write your description:")
        .with_help_message("Enter the detailed description of your commit")
        .prompt()
        .unwrap_or_else(|_| {
            println!("{}", "No description entered!".red());
            std::process::exit(1);
        });

    (title, content)
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

    let home = home_dir().ok_or("Home directory not found")?;
    let path = home.join(".config/quickgc/config.json");
    let config_path = path
        .to_str()
        .ok_or("Failed to convert path to string")?;

    let config = Config::from_file(config_path)?;
    let prefix = select_prefix(config.prefixes);
    let (title, content) = comment();
    handle_git_commit(&prefix, &title, &content);

    Ok(())
}
