use clap::Parser;
use noted::note::NoteError;
use noted::SortOrder;
use rusqlite::Connection;
use std::env;
use std::path::PathBuf;

mod args;
mod db;
mod helpers;

use args::*;
use db::*;
use helpers::*;

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

                create_new_note(&conn, note_content)?;
            } else if gui {
                create_note_from_gui(conn)?;
            } else if let Some(note_content) = content {
                create_new_note(&conn, note_content.join(" "))?;
            } else {
                return Err(NoteError::InputError(
                    "Provide either note content, --file or --gui".to_string(),
                ));
            }
        }

        Commands::All => {
            let notes = get_all_notes(&conn)?;
            print_notes(notes);
        }

        Commands::Last { count } => {
            let notes = get_notes_with_qty_and_order(&conn, count, SortOrder::Desc)?;
            print_notes(notes);
        }

        Commands::First { count } => {
            let notes = get_notes_with_qty_and_order(&conn, count, SortOrder::Asc)?;
            print_notes(notes);
        }

        Commands::Delete { id, all } => {
            if all {
                let prompt = "Are you sure you want to remove all notes?";
                let answer = read_y_or_no_input(prompt)?;
                match answer {
                    'y' => {
                        let count = delete_all_notes(&conn)?;
                        println!("Deleted {} notes", count);
                    }
                    _ => {
                        println!("Aborting");
                        return Ok(());
                    }
                };
            } else if let Some(to_be_deleted) = id {
                let count = delete_note(&conn, &to_be_deleted)?;
                println!(
                    "Deleted {} note(s) with ID starting with '{}'",
                    count, to_be_deleted
                );
            }
        }

        Commands::Search { term } => {
            let notes = search_notes(&conn, term)?;
            print_notes(notes)
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    struct TestDb {
        path: PathBuf,
    }

    impl TestDb {
        fn new(db_path: PathBuf) -> Self {
            TestDb { path: db_path }
        }

        fn conn(&self) -> Connection {
            Connection::open(&self.path).expect("Failed to open test database")
        }
    }

    impl Drop for TestDb {
        fn drop(&mut self) {
            if self.path.exists() {
                fs::remove_file(&self.path).expect("Failed to remove test database");
            }
        }
    }

    #[test]
    fn test_database_basics() {
        let home_dir = env::var("HOME").expect("Could not get $HOME directory");
        let db_path = PathBuf::from(home_dir).join(".local/share/noted/notes_test.db");

        let test_db = TestDb::new(db_path);

        // Test the connection to the database
        let conn = test_db.conn();
        assert!(
            conn.is_autocommit(),
            "Connection should be in autocommit mode"
        );

        // Test creating a table
        let create_table_result = create_table(&conn);
        assert!(
            create_table_result.is_ok(),
            "Creating the note table should not fail"
        );

        let first_note = String::from("A sneeze (also known as sternutation) is a semi-autonomous, convulsive expulsion of air from the lungs through the nose and mouth, usually caused by foreign particles irritating the nasal mucosa.");
        let second_note= String::from("A sneeze expels air forcibly from the mouth and nose in an explosive, spasmodic involuntary action. This action allows for mucus to escape through the nasal cavity and saliva to escape from the oral cavity.");

        {
            // Test first insert
            let insertion_result = create_new_note(&conn, first_note.to_string());
            assert!(
                insertion_result.is_ok(),
                "Creating a new note should not fail"
            );
        }
        {
            // Test second insert
            let insertion_result = create_new_note(&conn, second_note.to_string());
            assert!(
                insertion_result.is_ok(),
                "Creating a second note should not fail"
            );
        }

        {
            // Test getting the first note
            let mut result = get_notes_with_qty_and_order(&conn, 1, SortOrder::Asc).unwrap();
            let note = result.pop();
            if let Some(n) = note {
                assert_eq!(first_note, n.get_content());
            }
        }

        // Test getting the second note
        let mut result = get_notes_with_qty_and_order(&conn, 1, SortOrder::Desc).unwrap();
        let mut note = result.pop();
        if let Some(n) = note {
            assert_eq!(first_note, n.get_content());

            // Test searching for a note
            result = search_notes(&conn, "escape".to_string()).unwrap();
            assert_eq!(1, result.len());
            note = result.pop();
            if let Some(n) = note {
                assert!(n.get_content().contains("spasmodic involuntary action"));
            }

            // Test deleting the note
            let id = String::from(n.get_id());
            let deletion_result = delete_note(&conn, &id).unwrap();
            assert_eq!(1, deletion_result);
        }

        // There should now be only one note in the db
        result = get_all_notes(&conn).unwrap();
        assert_eq!(1, result.len());

        // Test clearing the database
        let deleted_rows = delete_all_notes(&conn).unwrap();
        assert_eq!(1, deleted_rows);
        let all_notes_after_reset = get_all_notes(&conn).unwrap();
        assert_eq!(0, all_notes_after_reset.len());
    }
}
