use anyhow::Result;
use db::*;
use noted::SortOrder;
use rusqlite::Connection;
use std::env;
use std::io::{self, Write};

mod db;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let cwd = env::current_dir().unwrap();
    let conn = Connection::open(cwd.join("notes.db"))?;

    match create_table(&conn) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Couldn't initialize the database: {}", e);
        }
    }

    if args.len() < 3 {
        println!("Instructions!");
    } else {
        match args[2].as_str() {
            "new" | "n" => {
                if args.len() < 4 {
                    println!("Create a new note by providing a note or a file, eg:");
                    println!("  noted new This here is a new note",);
                    println!("  noted new --file \"path-to-my-file.txt\"",);
                } else {
                    match args[3].as_str() {
                        "--file" | "-f" => {
                            if args.len() > 4 {
                                let filepath = &args[4];
                                let note_content = read_file(filepath)?;
                                return Ok(create_new_note(conn, note_content)?);
                            } else {
                                println!("Provide the file as an argument, eg:");
                                println!("  noted new --file \"path-to-my-file.txt\"",);
                            }
                        }
                        _ => {
                            let note_content = args[3..].join(" ");
                            return Ok(create_new_note(conn, note_content)?);
                        }
                    }
                }
            }
            "all" | "a" => {
                return Ok(get_all_notes(conn)?);
            }
            "last" | "l" => {
                if args.len() > 3 {
                    if let Ok(count) = args[3].parse::<i32>() {
                        return Ok(get_some_notes(conn, count, SortOrder::Desc)?);
                    }
                } else {
                    return Ok(get_some_notes(conn, 1, SortOrder::Desc)?);
                }
            }
            "first" | "f" => {
                if args.len() > 3 {
                    if let Ok(count) = args[3].parse::<i32>() {
                        return Ok(get_some_notes(conn, count, SortOrder::Asc)?);
                    }
                } else {
                    return Ok(get_some_notes(conn, 1, SortOrder::Asc)?);
                }
            }
            "delete" | "remove" | "d" | "rm" => {
                if args.len() < 4 {
                    println!(
                        "Specify which note you would like to remove by providing the note ID"
                    );
                    println!("You can also remove all notes with the option 'all'");
                } else {
                    match args[3].as_str() {
                        "a" | "all" => {
                            print!("Are you sure you want to remove all notes? [y]/[n]\n==> ");
                            let _ = io::stdout().flush();
                            let mut user_input = String::new();
                            io::stdin().read_line(&mut user_input)?;

                            match user_input.trim().chars().next() {
                                Some('y') => {
                                    conn.execute("DROP TABLE note", ())?;
                                }
                                _ => {
                                    println!("Cancelling");
                                }
                            }
                        }
                        _ => {
                            return Ok(delete_note(&conn, args[3].clone())?);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}
