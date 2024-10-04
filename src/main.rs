use anyhow::Result;
use noted::note::Note;
use rusqlite::Connection;
use std::env;
use std::fs::File;
use std::io::{self, Read};

enum SortOrder {
    Asc,
    Desc,
}

impl SortOrder {
    fn as_str(&self) -> &str {
        match self {
            SortOrder::Asc => "ASC",
            SortOrder::Desc => "DESC",
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let cwd = env::current_dir().unwrap();
    let conn = Connection::open(cwd.join("notes.db"))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS note (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            date  NUMBER NOT NULL
        )",
        (),
    )?;

    if args.len() < 3 {
        println!("Instructions!");
    } else {
        match args[2].as_str() {
            "new" | "n" => {
                let filepath = &args[3];
                return create_new_note(filepath, conn);
            }
            "all" | "a" => {
                return get_all_notes(conn);
            }
            "last" | "l" => {
                if args.len() > 3 {
                    if let Ok(count) = args[3].parse::<i32>() {
                        return get_some_notes(conn, count, SortOrder::Desc);
                    }
                } else {
                    return get_some_notes(conn, 1, SortOrder::Desc);
                }
            }
            "first" | "f" => {
                if args.len() > 3 {
                    if let Ok(count) = args[3].parse::<i32>() {
                        return get_some_notes(conn, count, SortOrder::Asc);
                    }
                } else {
                    return get_some_notes(conn, 1, SortOrder::Asc);
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    Ok(file_content)
}

fn create_new_note(path: &str, conn: Connection) -> Result<()> {
    let note_content = read_file(path)?;
    let new_note = Note::new(&note_content);

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

fn get_all_notes(conn: Connection) -> Result<()> {
    let mut statement = conn.prepare("SELECT id, content, date FROM note")?;

    let note_iterator = statement.query_map([], |row| {
        Ok(Note::from_db(row.get(0)?, row.get(1)?, row.get(2)?))
    })?;

    println!("\n");

    for note in note_iterator {
        println!("{}", note.unwrap().unwrap());
    }

    Ok(())
}

fn get_some_notes(conn: Connection, count: i32, order_by: SortOrder) -> Result<()> {
    let query = format!(
        "SELECT id, content, date FROM note ORDER BY date {} LIMIT ?1",
        order_by.as_str()
    );

    let mut statement = conn.prepare(&query)?;

    let note_iterator = statement.query_map([count], |row| {
        Ok(Note::from_db(row.get(0)?, row.get(1)?, row.get(2)?))
    })?;

    println!("\n");

    for note in note_iterator {
        println!("{}", note.unwrap().unwrap());
    }

    Ok(())
}
