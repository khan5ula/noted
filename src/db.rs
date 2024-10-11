use crate::helpers::{note_iter_into_vec, read_file_to_string};
use crate::note::Note;
use crate::note::NoteError;
use crate::SortOrder;
use chrono::Local;
use rusqlite::Connection;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

pub fn init_db() -> Result<Connection, NoteError> {
    let cwd = env::current_dir().unwrap();
    let home_dir = env::var("HOME").expect("Could not get $HOME directory");
    let db_path = PathBuf::from(home_dir).join(".local/share/noted/notes.db");
    let conn = Connection::open(cwd.join(db_path)).map_err(NoteError::RustqliteError)?;

    create_table(&conn)?;
    Ok(conn)
}

pub fn create_table(conn: &Connection) -> Result<(), NoteError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS note (
                    id TEXT PRIMARY KEY,
                    content TEXT NOT NULL,
                    date  NUMBER NOT NULL
                    )",
        (),
    )
    .map_err(NoteError::RustqliteError)?;

    Ok(())
}

pub fn create_new_note(conn: &Connection, content: String) -> Result<usize, NoteError> {
    let new_note = Note::new(content);

    conn.execute(
        "INSERT INTO note (id, content, date) VALUES (?1, ?2, ?3)",
        (
            &new_note.get_id().to_string(),
            &new_note.get_content(),
            &new_note.get_date(),
        ),
    )
    .map_err(NoteError::RustqliteError)
}

pub fn create_note_from_gui(conn: &Connection) -> Result<(), NoteError> {
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

        let note_content = read_file_to_string(&filename)?;

        create_new_note(conn, note_content)?;
        fs::remove_file(filename).map_err(NoteError::FileError)?;
    }

    Ok(())
}

pub fn get_all_notes(conn: &Connection) -> Result<Vec<Note>, NoteError> {
    let mut statement = conn
        .prepare("SELECT id, content, date FROM note")
        .map_err(NoteError::RustqliteError)?;

    let note_iterator = statement
        .query_map([], |row| {
            Ok(Note::from_db(row.get(0)?, row.get(1)?, row.get(2)?))
        })
        .map_err(NoteError::RustqliteError)?;

    note_iter_into_vec(note_iterator)
}

pub fn get_notes_with_qty_and_order(
    conn: &Connection,
    count: i32,
    order_by: SortOrder,
) -> Result<Vec<Note>, NoteError> {
    let query = format!(
        "SELECT id, content, date FROM note ORDER BY date {} LIMIT ?1",
        order_by.as_str()
    );

    let mut statement = conn.prepare(&query).map_err(NoteError::RustqliteError)?;

    let note_iterator = statement
        .query_map([count], |row| {
            Ok(Note::from_db(row.get(0)?, row.get(1)?, row.get(2)?))
        })
        .map_err(NoteError::RustqliteError)?;

    note_iter_into_vec(note_iterator)
}

pub fn delete_note(conn: &Connection, id: &String) -> Result<usize, NoteError> {
    let like_id = format!("{}%", id);

    conn.execute("DELETE FROM note WHERE id LIKE ?", [like_id])
        .map_err(NoteError::RustqliteError)
}

pub fn delete_all_notes(conn: &Connection) -> Result<usize, NoteError> {
    conn.execute("DELETE FROM note", ())
        .map_err(NoteError::RustqliteError)
}

pub fn search_notes_by_content(conn: &Connection, needle: &String) -> Result<Vec<Note>, NoteError> {
    let query = "SELECT id, content, date FROM note WHERE content LIKE ?";
    let search_with_wildcards = format!("%{}%", needle);

    let mut statement = conn.prepare(query).map_err(NoteError::RustqliteError)?;

    let note_iterator = statement
        .query_map([search_with_wildcards], |row| {
            Ok(Note::from_db(row.get(0)?, row.get(1)?, row.get(2)?))
        })
        .map_err(NoteError::RustqliteError)?;

    note_iter_into_vec(note_iterator)
}

pub fn search_notes_by_id(conn: &Connection, id: &String) -> Result<Vec<Note>, NoteError> {
    let like_id = format!("{}%", id);
    let query = "SELECT id, content, date FROM note WHERE id LIKE ?";
    let mut statement = conn.prepare(query).map_err(NoteError::RustqliteError)?;

    let note_iterator = statement
        .query_map([like_id], |row| {
            Ok(Note::from_db(row.get(0)?, row.get(1)?, row.get(2)?))
        })
        .map_err(NoteError::RustqliteError)?;

    note_iter_into_vec(note_iterator)
}

pub fn edit_note_with_gui(
    conn: &Connection,
    note_to_edit_path: &String,
    id: &String,
) -> Result<usize, NoteError> {
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
            format!("--filename={}", note_to_edit_path).as_str(),
        ])
        .output()
        .expect("Failed to execute yad command");

    if output.status.success() {
        let new_content_filepath = format!("/tmp/noted_new_note_{}", Local::now().timestamp());
        let mut file = File::create(&new_content_filepath).expect("Failed to create file");

        file.write_all(&output.stdout)
            .expect("Failed to write to file");

        let note_content = read_file_to_string(&new_content_filepath)?;

        fs::remove_file(new_content_filepath).map_err(NoteError::FileError)?;
        fs::remove_file(note_to_edit_path).map_err(NoteError::FileError)?;
        edit_note(conn, id, &note_content)
    } else {
        // Ensure the note to edit tmp file is removed if the yad operation is cancelled
        fs::remove_file(note_to_edit_path).map_err(NoteError::FileError)?;
        Ok(0)
    }
}

pub fn edit_note(conn: &Connection, id: &String, content: &String) -> Result<usize, NoteError> {
    let like_id = format!("{}%", id);
    let updated_timestamp = Note::create_new_timestamp().to_string();

    conn.execute(
        "UPDATE note SET content = ?1, date = ?2 WHERE id LIKE ?3",
        [content, &updated_timestamp, &like_id],
    )
    .map_err(NoteError::RustqliteError)
}

pub fn handle_edit_note(conn: &Connection, note: &Note) -> Result<usize, NoteError> {
    let filename = format!("/tmp/noted_note_to_edit_{}", Local::now().timestamp());
    let mut file = File::create(&filename).expect("Failed to create file");
    let buf = note.get_content().as_bytes();

    file.write_all(buf).expect("Failed to write to file");

    let id = note.get_id().to_string();

    edit_note_with_gui(conn, &filename, &id)
}
