use std::fmt;
use uuid::Uuid;
extern crate chrono;
use ansi_term::Colour::Blue;
use chrono::prelude::*;
use chrono::Local;
use rusqlite::Error;
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Debug)]
pub struct Note {
    id: String,
    content: String,
    date: i64,
}

impl Note {
    pub fn new(content: String) -> Self {
        Note {
            id: Uuid::new_v4().to_string(),
            content: content.to_string(),
            date: Local::now().timestamp(),
        }
    }

    pub fn from_db(id: String, content: String, date: i64) -> Result<Self, String> {
        match Uuid::parse_str(&id) {
            Ok(parsed_id) => Ok(Note {
                id: parsed_id.to_string(),
                content,
                date,
            }),
            Err(_) => Err(format!("Couldn't parse given uuid: {}", id)),
        }
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_date(&self) -> i64 {
        self.date
    }

    pub fn create_new_timestamp() -> i64 {
        Local::now().timestamp()
    }

    fn get_datetime(&self) -> String {
        let datetime = Local.timestamp_opt(self.date, 0);
        match datetime {
            chrono::LocalResult::Single(date) => date.format("%a %b %d. %H:%M:%S %Y").to_string(),
            chrono::LocalResult::None => "Invalid timestamp".to_string(),
            chrono::LocalResult::Ambiguous(_, _) => "Ambiguous timestamp".to_string(),
        }
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "_____ {} _____\n      {}      \n\n{}",
            Blue.bold().paint(Note::get_datetime(self)),
            Blue.paint(&self.id[0..25]),
            self.content
        )
    }
}

#[derive(Debug, Error)]
pub enum NoteError {
    #[error("Failed to unwrap note: {0}")]
    UnwrapNoteError(String),

    #[error("SQLite error: {0}")]
    RustqliteError(#[from] Error),

    #[error("File error: {0}")]
    FileError(#[from] IoError),

    #[error("Input error: {0}")]
    InputError(String),

    #[error("Unexpected result: {0}")]
    UnexpectedResultError(String),
}
