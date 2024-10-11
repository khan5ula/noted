use crate::note::{Note, NoteError};
use rusqlite::Error;
use std::fs::File;
use std::io::Read;
use std::io::{self, Write};

pub fn read_y_or_no_input(prompt: &str) -> Result<char, NoteError> {
    print!("{} [y]/[n]\n==> ", prompt);
    io::stdout()
        .flush()
        .map_err(|e| NoteError::InputError(e.to_string()))?;

    let mut user_input = String::new();
    let _ = io::stdin()
        .read_line(&mut user_input)
        .map_err(|e| NoteError::InputError(e.to_string()));

    let choice = user_input.trim().chars().next();
    match choice {
        Some(input) => match input.to_lowercase().next() {
            Some('y') => Ok('y'),
            Some('n') => Ok('n'),
            _ => {
                println!("Invalid input");
                Err(NoteError::InputError(
                    "Input must be 'y' or 'n'.".to_string(),
                ))
            }
        },
        None => {
            println!("No input provided.");
            Err(NoteError::InputError("No input provided.".to_string()))
        }
    }
}

pub fn read_file_to_string(path: &str) -> Result<String, NoteError> {
    let mut file = File::open(path).map_err(NoteError::FileError)?;
    let mut file_content = String::new();

    let _ = file
        .read_to_string(&mut file_content)
        .map_err(NoteError::FileError);

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
    let mut result: Vec<Note> = vec![];

    for iter_result in iterator {
        match iter_result {
            Ok(note_result) => match note_result {
                Ok(note) => {
                    result.push(note);
                }
                Err(e) => {
                    return Err(NoteError::UnwrapNoteError(e));
                }
            },
            Err(e) => {
                return Err(NoteError::RustqliteError(e));
            }
        }
    }

    Ok(result)
}
