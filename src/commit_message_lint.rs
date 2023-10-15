use regex::Regex;
use std::error::Error;
use std::fmt;

#[allow(dead_code)]
pub struct CommitMessage {
    kind: String,
    scope: Option<String>,
    subject: String,
    body: Option<String>,
    footer: Option<String>,
}

#[derive(Debug)]
pub enum CommitMessageError {
    EmptyType,
    InvalidTypeFormat(String),
    UpperCaseType,
    EmptySubject,
    SubjectEndsWithFullStop,
    HeaderExceedsMaxLength,
}

impl fmt::Display for CommitMessageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommitMessageError::EmptyType => write!(f, "Type is empty"),
            CommitMessageError::InvalidTypeFormat(pattern) => write!(f, "Invalid type format: {}", pattern),
            CommitMessageError::UpperCaseType => write!(f, "Type is uppercase"),
            CommitMessageError::EmptySubject => write!(f, "Subject is empty"),
            CommitMessageError::SubjectEndsWithFullStop => write!(f, "Subject ends with full stop"),
            CommitMessageError::HeaderExceedsMaxLength => write!(f, "Header exceeds max length"),
        }
    }
}

impl Error for CommitMessageError {}

impl CommitMessage {
    pub fn new(
        kind: String,
        scope: Option<String>,
        subject: String,
        body: Option<String>,
        footer: Option<String>,
    ) -> Self {
        Self {
            kind,
            scope,
            subject,
            body,
            footer,
        }
    }

    pub fn validate(&self) -> Result<(), CommitMessageError> {
        if self.kind.trim().is_empty() {
            return Err(CommitMessageError::EmptyType);
        }

        let re = Regex::new(r"^[a-z]+$").unwrap(); 
        if !re.is_match(&self.kind) {
            return Err(CommitMessageError::InvalidTypeFormat(r"^[a-z]+$".to_string()));
        }

        if self.kind.chars().any(|c| c.is_uppercase()) {
            return Err(CommitMessageError::UpperCaseType);
        }

        if self.subject.trim().is_empty() {
            return Err(CommitMessageError::EmptySubject);
        }

        if self.subject.ends_with('.') {
            return Err(CommitMessageError::SubjectEndsWithFullStop);
        }

        let header = format!("{}: {}", self.kind, self.subject);
        if header.len() > 100 {
            return Err(CommitMessageError::HeaderExceedsMaxLength);
        }

        Ok(())
    }
}
