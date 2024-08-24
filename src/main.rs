use colored::*;
use inquire::{Confirm, Select, Editor, Text,};
use dirs::home_dir;
use tokio::process::Command;
use tokio::io::{self, BufReader, AsyncBufReadExt};
use tokio::task;
use config::Config;
use commit_message_lint::CommitMessage;



mod config;
mod commit_message_lint;
mod render_config;

fn format_part(condition: bool, true_format: &str, false_format: &str) -> String {
    if condition {
        true_format.to_string()
    } else {
        false_format.to_string()
    }
}


async fn is_git_add() -> Result<bool, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .arg("status")
        .output()
        .await?;

    if !output.status.success() {
        eprintln!("Git command failed with {:?}", output.status);
        return Ok(false);
    }

    let stdout = String::from_utf8(output.stdout)?;
    Ok(!stdout.contains("Changes not staged for commit"))
}

async fn get_multiline_input(prompt: &str, help_message: &str) -> Result<String, Box<dyn std::error::Error>> {
    let prompt = prompt.to_string();
    let help_message = help_message.to_string();

    let input = task::spawn_blocking(move || {
        Editor::new(&prompt)
            .with_help_message(&help_message)
            .with_formatter(&|submission| {
                let char_count = submission.chars().count();
                if char_count == 0 {
                    String::from("<empty>")
                } else if char_count <= 50 {
                    submission.into()
                } else {
                    let mut substr: String = submission.chars().take(47).collect();
                    substr.push_str("...");
                    substr
                }
            })
            .prompt()
    }).await??;

    Ok(input)
}


async fn get_user_input(prompt: &str, help_message: &str) -> Result<String, Box<dyn std::error::Error>> {
    let prompt = prompt.to_string();
    let help_message = help_message.to_string();

    let input = task::spawn_blocking(move || {
        Text::new(&prompt)
            .with_help_message(&help_message)
            .prompt()

    }).await??;

    Ok(input)
}

async fn boolean_question(question: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let prompt = question.to_string();

    let input = task::spawn_blocking(move || {
        Confirm::new(&prompt)
            .with_default(false)
            .prompt()
    }).await??;
    Ok(input)
}

async fn select_kind(kinds: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    let kinds_clone = kinds.clone();
    let selection = task::spawn_blocking(move || {
        Select::new("Select git comment type", kinds).prompt()
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

    let index = kinds_clone.iter().position(|p| p == &selection).unwrap_or(0);
    let color = colors.get(index).unwrap_or(&Color::White);

    println!("{}", selection.color(*color));
    Ok(selection)
}

async fn handle_git_commit(params: (&str, &bool, &str, &str, &str, &str)) -> io::Result<()> {
    let (kind, break_changes, scope, subject, body, footer) = params;

    let scope_part = format_part(!scope.is_empty(), &format!("({})", scope), "");
    let is_break_change_header = format_part(*break_changes, "!", "");
    let is_break_change_foot = format_part(*break_changes, "[BREAKING CHANGE] ", "");

    let header = format!("{}{}{}: {}", kind, scope_part, is_break_change_header, subject);

    let optional_body = if body.is_empty() {
        String::new()
    } else {
        format!("\n\n{}", body)
    };
    let optional_footer = format!("\n{}{}", is_break_change_foot, footer);

    let mut child = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&header)
        .arg("-m")
        .arg(&optional_body)
        .arg("-m")
        .arg(&optional_footer)
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
        println!("\n{}", "🟢 Commit successful! ".black().on_green().bold());
        if let Some(stdout_content) = stdout {
            println!("\n{}", stdout_content.white().bold());
        }
    } else {
        println!("\n{}", "🔴 Commit failed! ".white().on_red().bold().italic());
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

    let files_staged = is_git_add().await?;

    if files_staged {
        let config = Config::from_file(config_path)?;
        let is_commit_lint_enabled = boolean_question("Enable commit lint? ").await?;
        let kind = select_kind(config.types).await?;
        let scope = get_user_input("Scope: ", "(optional)").await?;
        let is_break_changes = boolean_question("Is this a breaking change? ").await?;
        let subject = get_user_input("Subject: ", "must have").await?;
        let body = get_multiline_input("Body: ", "Enter commit body (optional, press Esc then Enter to finish)").await?;
        let footer = get_user_input("Footer: ", "(optional)").await?;

        let params = (&kind[..], &is_break_changes, &scope[..], &subject[..], &body[..], &footer[..]);

        if is_commit_lint_enabled {
            let commit_message = CommitMessage::new(
                kind.clone(),
                Some(scope.clone()),
                subject.clone(),
                Some(body.clone()),
                Some(footer.clone()),
            ).validate();

            match commit_message {
                Ok(_) => {
                    println!("{} {}", "🟢", " Commit message is valid. ".black().on_green().bold());
                    handle_git_commit(params).await?;
                }
                Err(e) => {
                    println!("{} {}", "🔴", format!(" Commit message is invalid: {}. ", e).white().on_red().bold());
                }
            }
        } else {
            handle_git_commit(params).await?;
        }

    } else {
        println!("\n{} {}", "🟠", " Unstaged file changes detected. ".bright_white().on_yellow().bold())
    }

    Ok(())
}
