use noted::note::Note;
use noted::note::NoteError;
use noted::SortOrder;
use rusqlite::Connection;
use rusqlite::Error;
use std::fs::File;
use std::io::Read;

pub fn create_table(conn: &Connection) -> Result<(), NoteError> {
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS note (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            date  NUMBER NOT NULL
            )",
        (),
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(NoteError::RustqliteError(e)),
    }
}

pub fn read_file(path: &str) -> Result<String, NoteError> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(NoteError::FileError(e.to_string())),
    };

    let mut file_content = String::new();

    match file.read_to_string(&mut file_content) {
        Ok(_) => Ok(file_content),
        Err(e) => Err(NoteError::FileError(e.to_string())),
    }
}

pub fn create_new_note(conn: &Connection, content: String) -> Result<(), NoteError> {
    let new_note = Note::new(content);

    match conn.execute(
        "INSERT INTO note (id, content, date) VALUES (?1, ?2, ?3)",
        (
            &new_note.get_id().to_string(),
            &new_note.get_content(),
            &new_note.get_date(),
        ),
    ) {
        Ok(_) => Ok(()),
        Err(_) => todo!(),
    }
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

fn note_iter_into_vec<I>(iterator: I) -> Result<Vec<Note>, NoteError>
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
                return Err(NoteError::IterationError(e));
            }
        }
    }

    Ok(result)
}

pub fn get_all_notes(conn: &Connection) -> Result<Vec<Note>, NoteError> {
    let mut statement = match conn.prepare("SELECT id, content, date FROM note") {
        Ok(statement) => statement,
        Err(e) => return Err(NoteError::RustqliteError(e)),
    };

    let note_iterator = match statement.query_map([], |row| {
        Ok(Note::from_db(row.get(0)?, row.get(1)?, row.get(2)?))
    }) {
        Ok(iterator) => iterator,
        Err(e) => return Err(NoteError::IterationError(e)),
    };

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

    let mut statement = match conn.prepare(&query) {
        Ok(statement) => statement,
        Err(e) => return Err(NoteError::RustqliteError(e)),
    };

    let note_iterator = match statement.query_map([count], |row| {
        Ok(Note::from_db(row.get(0)?, row.get(1)?, row.get(2)?))
    }) {
        Ok(iterator) => iterator,
        Err(e) => return Err(NoteError::IterationError(e)),
    };

    note_iter_into_vec(note_iterator)
}

pub fn delete_note(conn: &Connection, id: &String) -> Result<usize, NoteError> {
    let like_id = format!("{}%", id);

    match conn.execute("DELETE FROM note WHERE id LIKE ?", [like_id]) {
        Ok(rows_deleted) => Ok(rows_deleted),
        Err(e) => {
            println!(
                "Couldn't remove a note with given id: '{}' due to: {}",
                id, e
            );
            Err(NoteError::RustqliteError(e))
        }
    }
}

pub fn delete_all_notes(conn: &Connection) -> Result<usize, NoteError> {
    match conn.execute("DELETE FROM note", ()) {
        Ok(count) => Ok(count),
        Err(e) => Err(NoteError::RustqliteError(e)),
    }
}

pub fn search_notes(conn: &Connection, needle: String) -> Result<Vec<Note>, NoteError> {
    let query = "SELECT id, content, date FROM note WHERE content LIKE ?";
    let search_with_wildcards = format!("%{}%", needle);

    let mut statement = match conn.prepare(query) {
        Ok(statement) => statement,
        Err(e) => return Err(NoteError::RustqliteError(e)),
    };

    let note_iterator = match statement.query_map([search_with_wildcards], |row| {
        Ok(Note::from_db(row.get(0)?, row.get(1)?, row.get(2)?))
    }) {
        Ok(iterator) => iterator,
        Err(e) => return Err(NoteError::IterationError(e)),
    };

    note_iter_into_vec(note_iterator)
}
