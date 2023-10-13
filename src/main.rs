extern crate colored;
extern crate dirs;
extern crate inquire;
extern crate serde;

use colored::*;
use config::Config;
use dirs::home_dir;
use tokio::process::Command;
use tokio::task;

mod config;
mod render_config;

async fn select_prefix(prefixes: Vec<String>) -> String {
    let prefixes_clone = prefixes.clone();
    let selection = task::spawn_blocking(move || {
        inquire::Select::new("Select git comment prefix", prefixes.clone()).prompt()
    }).await.expect("Failed to run blocking code");

    match selection {
        Ok(prefix) => {
            let index = prefixes_clone.iter().position(|p| p == &prefix).unwrap_or(0);
            match index {
                0 => println!("{}", prefix.blue()),
                1 => println!("{}", prefix.magenta()),
                2 => println!("{}", prefix.yellow()),
                3 => println!("{}", prefix.cyan()),
                4 => println!("{}", prefix.red()),
                5 => println!("{}", prefix.green()),
                6 => println!("{}", prefix.bright_purple()),
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

async fn comment() -> (String, String) {
    let title = task::spawn_blocking(|| {
        inquire::Text::new("Write your comment:")
            .with_help_message("Enter the title of your commit")
            .prompt()
    }).await.expect("Failed to run blocking code")
        .unwrap_or_else(|_| {
            println!("{}", "No comment entered!".red());
            std::process::exit(1);
        });


    let content = task::spawn_blocking(|| { 
        inquire::Text::new("Write your description:")
            .with_help_message("Enter the detailed description of your commit")
            .prompt()
    }).await.expect("Failed to run blocking code")
        .unwrap_or_else(|_| {
            println!("{}", "No description entered!".red());
            std::process::exit(1);
        });

    (title, content)
}

async fn handle_git_commit(prefix: &str, title: &str, content: &str) {
    let commit_message = format!("{} {}\n\n{}", prefix, title, content);
    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&commit_message)
        .output()
        .await
        .expect("Failed to execute git commit");

    println!("{}", String::from_utf8_lossy(&output.stdout).bright_cyan());

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout).white());
        println!("{}", "Commit successful!".white().on_green().bold());
    } else {
        println!("{}", String::from_utf8_lossy(&output.stderr).white());
        println!("{}", "Commit failed!".white().on_red().bold().italic());
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    render_config::setup_inquire()?;

    let home = home_dir().ok_or("Home directory not found")?;
    let path = home.join(".config/quickgc/config.json");
    let config_path = path
        .to_str()
        .ok_or("Failed to convert path to string")?;

    let config = Config::from_file(config_path)?;
    let prefix = select_prefix(config.prefixes).await;
    let (title, content) = comment().await;
    handle_git_commit(&prefix, &title, &content).await;

    Ok(())
}
