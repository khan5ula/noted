use crate::note::{Note, NoteError};
use rusqlite::Error;
use std::fs::File;
use std::io::Read;
use std::io::{self, Write};

pub fn read_y_or_no_input(prompt: &str) -> Result<char, NoteError> {
    print!("{} [y]/[n]\n==> ", prompt);
    io::stdout().flush().map_err(NoteError::IoError)?;

    let mut user_input = String::new();
    let _ = io::stdin()
        .read_line(&mut user_input)
        .map_err(NoteError::IoError);

    let choice = user_input.trim().chars().next();

    if let Some(input) = choice {
        match input.to_lowercase().next() {
            Some('y') => Ok('y'),
            Some('n') => Ok('n'),
            _ => {
                println!("Invalid input");
                Err(NoteError::UnexpectedResultError(
                    "Input must be 'y' or 'n'.".to_string(),
                ))
            }
        }
    } else {
        Ok('n')
    }
}

pub fn read_file_to_string(path: &str) -> Result<String, NoteError> {
    let mut file = File::open(path).map_err(NoteError::IoError)?;
    let mut file_content = String::new();

    let _ = file
        .read_to_string(&mut file_content)
        .map_err(NoteError::IoError);

    Ok(file_content)
}

pub fn print_notes(notes: Vec<Note>) {
    if notes.is_empty() {
        println!("No notes to display");
    }

    for note in notes {
        println!("{}", note);
        if !note.get_content().ends_with('\n') {
            println!("\n");
        }
    }
}

pub fn note_iter_into_vec<I>(iterator: I) -> Result<Vec<Note>, NoteError>
where
    I: IntoIterator<Item = Result<Result<Note, String>, Error>>,
{
    iterator
        .into_iter()
        .map(|iter_result| {
            iter_result
                .map_err(NoteError::RustqliteError)?
                .map_err(NoteError::UnwrapNoteError)
        })
        .collect()
}
