use clap::Parser;
use db::*;
use helpers::*;
use noted::note::NoteError;
use noted::SortOrder;
use rusqlite::Connection;
use std::env;
use std::path::PathBuf;

mod args;
mod db;
mod helpers;

use args::*;

fn main() -> Result<(), NoteError> {
    let cwd = env::current_dir().unwrap();
    let home_dir = env::var("HOME").expect("Could not get $HOME directory");
    let db_path = PathBuf::from(home_dir).join(".local/share/noted/notes.db");

    let conn = match Connection::open(cwd.join(db_path)) {
        Ok(conn) => conn,
        Err(e) => return Err(NoteError::RustqliteError(e)),
    };

    match create_table(&conn) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Couldn't initialize the database: {}", e);
        }
    }

    let args = NotedArgs::parse();

    match args.command {
        Commands::New { content, file, gui } => {
            if let Some(file) = file {
                let note_content = match read_file(&file) {
                    Ok(note_content) => note_content,
                    Err(e) => return Err(NoteError::FileError(e.to_string())),
                };

                create_new_note(conn, note_content)?;
            } else if gui {
                create_note_from_gui(conn)?;
            } else if let Some(note_content) = content {
                create_new_note(conn, note_content.join(" "))?;
            } else {
                eprintln!("Error: You must provide either note content, --file or --gui");
            }
        }

        Commands::All => {
            get_all_notes(conn)?;
        }

        Commands::Last { count } => {
            get_some_notes(conn, count, SortOrder::Desc)?;
        }

        Commands::First { count } => {
            get_some_notes(conn, count, SortOrder::Asc)?;
        }

        Commands::Delete { id } => {
            handle_delete(id, conn)?;
        }

        Commands::Search { term } => {
            find_notes(&conn, term)?;
        }
    }

    Ok(())
}
