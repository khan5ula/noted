use anyhow::Result;
use noted::note::Note;
use noted::SortOrder;
use rusqlite::Connection;
use std::fs::File;
use std::io::{self, Read};

pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS note (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            date  NUMBER NOT NULL
            )",
        (),
    )?;
    Ok(())
}

pub fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    Ok(file_content)
}

pub fn create_new_note(conn: Connection, content: String) -> Result<()> {
    let new_note = Note::new(content);

    conn.execute(
        "insert into note (id, content, date) values (?1, ?2, ?3)",
        (
            &new_note.get_id().to_string(),
            &new_note.get_content(),
            &new_note.get_date(),
        ),
    )?;

    Ok(())
}

pub fn get_all_notes(conn: Connection) -> Result<()> {
    let mut statement = conn.prepare("SELECT id, content, date FROM note")?;

    let note_iterator = statement.query_map([], |row| {
        Ok(Note::from_db(row.get(0)?, row.get(1)?, row.get(2)?))
    })?;

    let mut notes_found = false;

    for iter_result in note_iterator {
        match iter_result {
            Ok(note_result) => match note_result {
                Ok(note) => {
                    notes_found = true;
                    println!("{}", note);
                }
                Err(e) => {
                    println!("Couldn't unwrap note: {}", e);
                }
            },
            Err(e) => {
                println!("Couldn't iterate through notes: {}", e);
            }
        }
    }

    if !notes_found {
        println!("No notes to display");
    }

    Ok(())
}

pub fn get_some_notes(conn: Connection, count: i32, order_by: SortOrder) -> Result<()> {
    let query = format!(
        "SELECT id, content, date FROM note ORDER BY date {} LIMIT ?1",
        order_by.as_str()
    );

    let mut statement = conn.prepare(&query)?;

    let note_iterator = statement.query_map([count], |row| {
        Ok(Note::from_db(row.get(0)?, row.get(1)?, row.get(2)?))
    })?;

    let mut notes_found = false;

    for iter_result in note_iterator {
        match iter_result {
            Ok(note_result) => match note_result {
                Ok(note) => {
                    notes_found = true;
                    println!("{}", note);
                }
                Err(e) => {
                    println!("Couldn't unwrap note: {}", e);
                }
            },
            Err(e) => {
                println!("Couldn't iterate through notes: {}", e);
            }
        }
    }

    if !notes_found {
        println!("No notes to display");
    }

    Ok(())
}

pub fn delete_note(conn: &Connection, id: String) -> Result<()> {
    let like_id = format!("{}%", id);

    match conn.execute("DELETE FROM note WHERE id LIKE ?", [like_id]) {
        Ok(rows_deleted) => {
            println!(
                "Deleted {} note(s) with ID starting with '{}'",
                rows_deleted, id
            );
        }
        Err(e) => {
            println!(
                "Couldn't remove a note with given id: '{}' due to: {}",
                id, e
            );
        }
    }

    Ok(())
}
