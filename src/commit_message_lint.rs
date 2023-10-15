use regex::Regex;
use std::fmt;

pub struct CommitMessage {
    kind: String,
    scope: Option<String>,
    subject: String,
    body: Option<String>,
    footer: Option<String>,
}

impl fmt::Display for CommitMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)?;
        if let Some(scope) = &self.scope {
            write!(f, "({})", scope)?;
        }
        write!(f, ": {}", self.subject)?;
        if let Some(body) = &self.body {
            write!(f, "\n\n{}", body)?;
        }
        if let Some(footer) = &self.footer {
            write!(f, "\n\n{}", footer)?;
        }
        Ok(())
    }
}

impl CommitMessage {
    pub fn new(kind: String, scope: Option<String>, subject: String, body: Option<String>, footer: Option<String>) -> Self {
        Self { kind, scope, subject, body, footer }
    }

   pub fn is_valid(&self) -> bool {
        self.is_type_lowercase()
            && self.is_type_non_empty()
            // && self.is_subject_case_valid()
            && self.is_subject_non_empty()
            && !self.is_subject_end_with_full_stop()
            && self.is_header_max_length()
    }

    fn is_type_lowercase(&self) -> bool {
        self.kind == self.kind.to_lowercase()
    }

    fn is_type_non_empty(&self) -> bool {
        !self.kind.is_empty()
    }

    // fn is_subject_case_valid(&self) -> bool {
    //     let re = Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap();
    //     re.is_match(&self.subject)
    // }

    fn is_subject_non_empty(&self) -> bool {
        !self.subject.is_empty()
    }

    fn is_subject_end_with_full_stop(&self) -> bool {
        self.subject.ends_with('.')
    }

    fn is_header_max_length(&self) -> bool {
        let header = format!("{}", self);
        header.len() <= 100
    }

}
