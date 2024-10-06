use ansi_term::Style;
use anyhow::Result;
use db::*;
use noted::note::NoteError;
use noted::SortOrder;
use rusqlite::Connection;
use std::fs::File;
use std::io::{self, Write};
use std::process::Command;
use std::{env, fs};

mod db;

fn main() -> Result<(), NoteError> {
    let args: Vec<String> = env::args().collect();
    let cwd = env::current_dir().unwrap();

    let conn = match Connection::open(cwd.join("notes.db")) {
        Ok(conn) => conn,
        Err(e) => return Err(NoteError::RustqliteError(e)),
    };

    match create_table(&conn) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Couldn't initialize the database: {}", e);
        }
    }

    if args.len() < 3 {
        print_instructions();
    } else {
        match args[2].as_str() {
            "new" | "n" => {
                handle_new(args, conn)?;
            }
            "all" | "a" => return get_all_notes(conn),
            "last" | "l" => {
                if args.len() > 3 {
                    if let Ok(count) = args[3].parse::<i32>() {
                        get_some_notes(conn, count, SortOrder::Desc)?;
                    }
                } else {
                    get_some_notes(conn, 1, SortOrder::Desc)?;
                }
            }
            "first" | "f" => {
                if args.len() > 3 {
                    if let Ok(count) = args[3].parse::<i32>() {
                        get_some_notes(conn, count, SortOrder::Asc)?;
                    }
                } else {
                    get_some_notes(conn, 1, SortOrder::Asc)?;
                }
            }
            "delete" | "remove" | "d" | "rm" => {
                handle_delete(args, conn)?;
            }
            "search" | "s" => {
                if args.len() < 4 {
                    println!("Provide the search term");
                } else {
                    let needle = &args[3];
                    find_notes(&conn, needle.to_string())?;
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn handle_new(args: Vec<String>, conn: Connection) -> Result<(), NoteError> {
    if args.len() < 4 {
        println!("Create a new note by providing a note or a file, eg:");
        println!("  noted new This here is a new note",);
        println!("  noted new --file \"path-to-my-file.txt\"",);
    } else {
        match args[3].as_str() {
            "--file" | "-f" => {
                if args.len() > 4 {
                    let filepath = &args[4];

                    let mut note_content = match read_file(filepath) {
                        Ok(note_content) => note_content,
                        Err(e) => return Err(NoteError::FileError(e.to_string())),
                    };

                    if !note_content.ends_with('\n') {
                        note_content.push('\n');
                    }
                    create_new_note(conn, note_content)?;
                } else {
                    println!("Provide the file as an argument, eg:");
                    println!("  noted new --file \"path-to-my-file.txt\"",);
                }
            }
            "--gui" | "-g" => {
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
                    let mut file = File::create("new_note.txt").expect("Failed to create file");
                    file.write_all(&output.stdout)
                        .expect("Failed to write to file");

                    let mut note_content = match read_file("new_note.txt") {
                        Ok(note_content) => note_content,
                        Err(e) => return Err(NoteError::FileError(e.to_string())),
                    };

                    if !note_content.ends_with('\n') {
                        note_content.push('\n');
                    }

                    match create_new_note(conn, note_content) {
                        Ok(()) => match fs::remove_file("new_note.txt") {
                            Ok(()) => {}
                            Err(e) => return Err(NoteError::FileError(e.to_string())),
                        },
                        Err(e) => return Err(NoteError::FileError(e.to_string())),
                    };
                }
            }
            _ => {
                let mut note_content = args[3..].join(" ");
                if !note_content.ends_with('\n') {
                    note_content.push('\n');
                }
                create_new_note(conn, note_content)?;
            }
        }
    }

    Ok(())
}

fn handle_delete(args: Vec<String>, conn: Connection) -> Result<(), NoteError> {
    if args.len() < 4 {
        println!("Specify which note you would like to remove by providing the note ID");
        println!("You can also remove all notes with the option 'all'");
    } else {
        match args[3].as_str() {
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
                                return Err(NoteError::InputError(
                                    "Invalid user input".to_string(),
                                ));
                            }
                        }
                    }
                    Err(e) => return Err(NoteError::InputError(e.to_string())),
                }
            }
            _ => {
                delete_note(&conn, args[3].clone())?;
            }
        }
    }
    Ok(())
}

fn print_instructions() {
    println!("Create, read and manage quick notes");
    println!("\nUsage: noted COMMAND [OPTIONS]");
    println!("\n{}", Style::new().bold().paint("Commands:"));
    println!("  new (n)\t\tCreate a new note");
    println!("  all (a)\t\tView all notes");
    println!("  last (l)\t\tView recent note(s)");
    println!("  first (f)\t\tView oldest note(s)");
    println!("  delete (d,rm,remove)\tDelete note(s)");
    println!("  search (s)\t\tSearch for notes with certain keyword");
    println!("\n{}", Style::new().bold().paint("New options:"));
}
