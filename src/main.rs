use colored::*;
use config::Config;
use dirs::home_dir;
use tokio::process::Command;
use tokio::io::{self, BufReader, AsyncBufReadExt};
use tokio::task;

mod config;
mod render_config;

async fn get_user_input(prompt: &str, help_message: &str) -> Result<String, Box<dyn std::error::Error>> {
    let prompt = prompt.to_string();
    let help_message = help_message.to_string();

    let input = task::spawn_blocking(move || {
        inquire::Text::new(&prompt)
            .with_help_message(&help_message)
            .prompt()
    }).await??;

    Ok(input)
}

async fn is_breaking_change() -> Result<bool, Box<dyn std::error::Error>> {
    let input = task::spawn_blocking(move || {
        inquire::Confirm::new("Is this a breaking change?")
            .with_default(false)
            .prompt()
    }).await??;
    Ok(input)
}

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

async fn handle_git_commit(kind: &str, break_changes: &bool, scope: &str, subject: &str, body: &str) -> io::Result<()> {
    let scope_part = if !scope.is_empty() {
        format!("({})", scope)
    } else {
        String::new()
    };

    let is_break_change = match &break_changes {
        true => format!("!"),
        _ => String::new(),
    };

    let commit_message = format!("[{}{}{}] {}", &kind, scope_part, is_break_change, subject);
    let optional_body = format!("\n{}", body);
    let mut child = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&commit_message)
        .arg("-m")
        .arg(&optional_body)
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
    let kind = select_prefix(config.prefixes).await?;
    let scope = get_user_input("Scope", "Scope of the change (optional)").await?;
    let is_break_changes = is_breaking_change().await?;
    let subject = get_user_input("Subject", "Subject of the change").await?;
    let body = get_user_input("Body", "Body of the change (optional)").await?;
    handle_git_commit(&kind, &is_break_changes, &scope, &subject, &body).await?;

    Ok(())
}
