use crate::db::*;
use chrono::Local;
use noted::note::NoteError;
use rusqlite::Connection;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::process::Command;

pub fn create_note_from_gui(conn: Connection) -> Result<(), NoteError> {
    let output = Command::new("yad")
        .args([
            "--text-info",
            "--editable",
            "--wrap",
            "--show-uri",
            "--save-file",
            "--margins=20",
            "--show-uri",
            "--indent",
            "--brackets",
            "--title=Noted - New Note",
            "--width=900",
            "--height=800",
            "--button=Cancel (Esc)!gtk-cancel:1",
            "--button=Submit (Ctrl+Enter)!gtk-ok:0",
        ])
        .output()
        .expect("Failed to execute yad command");

    if output.status.success() {
        let filename = format!("/tmp/noted_new_note_{}", Local::now().timestamp());
        let mut file = File::create(&filename).expect("Failed to create file");

        file.write_all(&output.stdout)
            .expect("Failed to write to file");

        let note_content = match read_file(&filename) {
            Ok(note_content) => note_content,
            Err(e) => {
                return Err(NoteError::FileError(format!(
                    "Failed to read note from a file: {}",
                    e
                )))
            }
        };

        create_new_note(&conn, note_content)?;
        fs::remove_file(filename).map_err(|e| NoteError::FileError(e.to_string()))?
    }

    Ok(())
}

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
