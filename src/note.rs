use std::fmt;
use uuid::Uuid;
extern crate chrono;
use ansi_term::Colour::Blue;
use chrono::prelude::*;
use chrono::Local;
use rusqlite::Error;
use std::error::Error as StdError;

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

#[derive(Debug)]
pub enum NoteError {
    IterationError(Error),
    UnwrapNoteError(String),
    RustqliteError(Error),
    FileError(String),
    InputError(String),
    UnexpectedResultError(String),
}

impl fmt::Display for NoteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NoteError::IterationError(e) => {
                write!(f, "Couldn't iterate through notes: {}", e)
            }
            NoteError::UnwrapNoteError(e) => write!(f, "Couldn't unwrap note: {}", e),
            NoteError::RustqliteError(e) => {
                write!(f, "Rustqlite error while handling notes: {}", e)
            }
            NoteError::FileError(e) => {
                write!(f, "Error occured while trying to parse a file: {}", e)
            }
            NoteError::InputError(e) => {
                write!(f, "Error while reading user input: {}", e)
            }
            NoteError::UnexpectedResultError(e) => {
                write!(f, "Received unexpected result: {}", e)
            }
        }
    }
}

impl StdError for NoteError {}
