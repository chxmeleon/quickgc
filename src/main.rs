extern crate clap;
extern crate colored;
extern crate inquire;

use colored::*;
use inquire::Select;
use std::io;
use std::process::Command;

fn select_prefix() -> String {
    let prefixes: Vec<&str> = vec![
        "[FEATURE]",
        "[BUGFIX]",
        "[BUILD]",
        "[STYLE]",
        "[REFACTOR]",
        "[DOCS]",
        "[CODEREVIEW]",
    ];

    let selection = Select::new("Select git comment prefix", prefixes).prompt();

    match selection {
        Ok(prefix) => {
            match prefix {
                "[FEATURE]" => println!("{}", prefix.blue()),
                "[BUGFIX]" => println!("{}", prefix.magenta()),
                "[BUILD]" => println!("{}", prefix.yellow()),
                "[STYLE]" | "[CODEREVIEW]" => println!("{}", prefix.cyan()),
                "[REFACTOR]" => println!("{}", prefix.red()),
                "[DOCS]" => println!("{}", prefix.green()),
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
    println!("{}", "Write your comment:".blue().bold());
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");

    let mut content = String::new();
    println!("{}", "Write your description:".yellow().bold());
    io::stdin()
        .read_line(&mut content)
        .expect("Failed to read line");

    (title.trim().to_string(), content.trim().to_string())
}

fn handle_git_commit(prefix: &str, title: &str, content: &str) {
    let commit_message = format!("{} {}\n\n{}", prefix, title, content);
    let status = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&commit_message)
        .status()
        .expect("Failed to execute git commit");

    if status.success() {
        println!("{}", "Commit successful!".green().bold());
    } else {
        println!("{}", "Commit failed!".red().bold());
    }
}

fn main() {
    let prefix = select_prefix();
    let (title, content) = comment();
    handle_git_commit(&prefix, &title, &content);
}
