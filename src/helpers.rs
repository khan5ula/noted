use crate::note::{Note, NoteError};
use rusqlite::Error;
use std::fs::File;
use std::io::Read;

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
