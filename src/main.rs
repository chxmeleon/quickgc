use colored::*;
use config::Config;
use dirs::home_dir;
use tokio::process::Command;
use tokio::io::{self, BufReader, AsyncBufReadExt};
use tokio::task;

mod config;
mod render_config;

async fn select_prefix(prefixes: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    let prefixes_clone = prefixes.clone();
    let selection = task::spawn_blocking(move || {
        inquire::Select::new("Select git comment prefix", prefixes).prompt()
    }).await??;

    let colors = [
        Color::Blue,
        Color::Magenta,
        Color::Yellow,
        Color::Cyan,
        Color::Red,
        Color::Green,
        Color::BrightMagenta,
        Color::White,
    ];

    let index = prefixes_clone.iter().position(|p| p == &selection).unwrap_or(0);
    let color = colors.get(index).unwrap_or(&Color::White);

    println!("{}", selection.color(*color));
    Ok(selection)
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

async fn handle_git_commit(prefix: &str, title: &str, content: &str) -> io::Result<()> {
    let commit_message = format!("{} {}\n\n{}", prefix, title, content);
    let mut child = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&commit_message)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    let stderr = if let Some(stderr) = child.stderr.take() {
        let mut reader = BufReader::new(stderr).lines();
        let mut stderr_content = String::new();

        while let Some(line) = reader.next_line().await? {
            stderr_content.push_str(&line);
            stderr_content.push('\n');
            println!("{}", line.bright_cyan());
        }

        Some(stderr_content)
    } else {
        None
    };

    let stdout = if let Some(stdout) = child.stdout.take() {
        let mut reader = BufReader::new(stdout).lines();
        let mut stdout_content = String::new();

        while let Some(line) = reader.next_line().await? {
            stdout_content.push_str(&line);
            stdout_content.push('\n');
        }

        Some(stdout_content)
    } else {
        None
    };

    let status = child.wait().await?;

    if status.success() {
        println!("\n{}", "Commit successful!".black().on_green().bold());
        if let Some(stdout_content) = stdout {
            println!("\n{}", stdout_content.white().bold());
        }
    } else {
        println!("\n{}", "Commit failed!".white().on_red().bold().italic());
        if let Some(stderr_content) = stderr {
            println!("Error:\n{}", stderr_content.white().on_red().bold());
        }
    }

    Ok(())
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
    let prefix = select_prefix(config.prefixes).await?;
    let (title, content) = comment().await;
    handle_git_commit(&prefix, &title, &content).await?;

    Ok(())
}
