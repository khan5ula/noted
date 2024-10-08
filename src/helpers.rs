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
            "--margins=10",
            "--show-uri",
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

        let mut note_content = match read_file(&filename) {
            Ok(note_content) => note_content,
            Err(e) => {
                return Err(NoteError::FileError(format!(
                    "Failed to read note from a file: {}",
                    e
                )))
            }
        };

        if !note_content.ends_with('\n') {
            note_content.push('\n');
        }

        match create_new_note(conn, note_content) {
                        Ok(()) => match fs::remove_file(filename) {
                            Ok(()) => {}
                            Err(e) => return Err(NoteError::FileError(format!("Failed to remove the temporary file used when creating a new note from GUI: {}", e))),
                        },
                        Err(e) => return Err(NoteError::FileError(format!("Failed to create a new note from GUI: {}", e))),
                    };
    }

    Ok(())
}

pub fn handle_delete(id: String, conn: Connection) -> Result<(), NoteError> {
    match id.as_str() {
        "a" | "all" => {
            print!("Are you sure you want to remove all notes? [y]/[n]\n==> ");
            let _ = io::stdout().flush();
            let mut user_input = String::new();

            match io::stdin().read_line(&mut user_input) {
                Ok(_) => {
                    let choice = user_input.trim().chars().next();

                    match choice {
                        Some('y') | Some('Y') => {
                            if let Err(e) = conn.execute("DROP TABLE note", ()) {
                                Err(NoteError::RustqliteError(e))?;
                            }
                            println!("All notes removed");
                        }
                        Some('n') | Some('N') => {
                            println!("Cancelling");
                        }
                        _ => {
                            println!("Invalid input. Please enter 'y' or 'n'.");
                            return Err(NoteError::InputError("Invalid user input".to_string()));
                        }
                    }
                }
                Err(e) => return Err(NoteError::InputError(e.to_string())),
            }
        }
        _ => {
            delete_note(&conn, id)?;
        }
    }
    Ok(())
}
