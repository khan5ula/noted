use crate::db::{
    create_new_note, create_note_from_gui, delete_all_notes, delete_note, get_all_notes,
    get_notes_with_qty_and_order, handle_edit_note, search_notes_by_content, search_notes_by_id,
};
use crate::helpers::{print_notes, read_file_to_string, read_y_or_no_input};
use crate::note::NoteError;
use ansi_term::Colour::Blue;
use clap::{Parser, Subcommand};
use noted::SortOrder;
use rusqlite::Connection;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct NotedArgs {
    /// Subcommands
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new note (n)
    #[command(
        alias = "n",
        after_long_help = format!("Add a new note with command {}. New can be shortened to n. Provide the new note either directly after the command, eg. {} or use {} or {} subcommands to provide the note either as a text file or with graphical user interface.", 
            Blue.paint("new"), 
            Blue.paint("noted new Remember to buy some more coffee ASAP"),
            Blue.paint("--file"), 
            Blue.paint("--gui"))
    )]
    New {
        /// Note content
        #[arg(
            help = "Content of the new note",
            long_help = "Provide the content of the new note as a string. If you do not specify a file or GUI option, this argument is required.",
            required_unless_present_any(&["file", "gui"])
        )]
        content: Option<Vec<String>>,

        /// Create note from a file
        #[arg(
            short,
            long,
            help = "Create note from file",
            long_help = format!("Provide the content of the new note as a text file, eg. Noted new --file my_note.txt")
        )]
        file: Option<String>,

        /// Open GUI to create note
        #[arg(
            short,
            long,
            help = "Create note via GUI",
            long_help = "In order to create multi-line notes, Noted allows writing a note with a graphical editor. Enable graphical editor by: Noted new --gui"
        )]
        gui: bool,
    },

    /// View all notes (a)
    #[command(
        alias = "a",
        after_long_help = "View all notes. Command all can be shortened to a."
    )]
    All,

    /// View newest notes
    #[command(alias = "l")]
    Last {
        /// Number of recent notes to view (default is 1)
        #[arg(
            default_value_t = 1,
            help = "Number of recent notes to view",
            long_help = "Provide a number of recent notes you would like to see, eg. Noted last 4, which returns 4 latest notest in descending order. Defaults to 1 if count is not provided"
        )]
        count: i32,
    },

    /// View oldest notes
    #[command(alias = "f")]
    First {
        /// Number of oldest notes to view (default is 1)
        #[arg(
            default_value_t = 1,
            help = "Number of old notes to view",
            long_help = "Provide a number of the oldest notes you would like to see, eg. Noted first 4, which returns 4 first notest in ascending order. Defaults to 1 if count is not provided"
        )]
        count: i32,
    },

    /// Delete a note by ID
    #[command(aliases = ["d", "rm", "remove"])]
    Delete {
        #[arg(
            help = "ID of the note to delete",
            long_help = "Delete note(s) by providing a note ID. You can also use the beginning of an ID, eg. f1a8. The command removes all notes with a matching start of an ID. All notes can be deleted with option --all (-a)"
        )]
        id: Option<String>,

        #[arg(short = 'a', long = "all", help = "Delete all notes")]
        all: bool,
    },

    /// Search for a note
    #[command(alias = "s")]
    Search {
        #[arg(help = "Search term to find notes")]
        term: String,
    },

    /// Edit a note by ID
    #[command(alias = "e")]
    Edit {
        #[arg(
            help = "ID of the note to edit",
            long_help = "Edit a note by providing a note ID. You can also use the beginning of an ID, eg. f1a8. If only one note is found with the given ID, editing is allowed."
        )]
        id: String,
    },
}

pub fn handle_args(conn: Connection, args: NotedArgs) -> Result<(), NoteError> {
    match args.command {
        Commands::New { content, file, gui } => {
            if let Some(file) = file {
                let note_content = match read_file_to_string(&file) {
                    Ok(note_content) => note_content,
                    Err(e) => return Err(NoteError::FileError(e.to_string())),
                };

                create_new_note(&conn, note_content)?;
            } else if gui {
                create_note_from_gui(&conn)?;
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
            let notes = search_notes_by_content(&conn, &term)?;
            print_notes(notes)
        }

        Commands::Edit { id } => {
            let mut search_results = search_notes_by_id(&conn, &id)?;
            if search_results.len() > 1 {
                println!(
                    "Found {} results with given id {}, editing requires only 1 search result",
                    &search_results.len(),
                    &id
                );
            } else {
                let note = search_results.pop();
                if let Some(note_to_edit) = note {
                    match handle_edit_note(&conn, &note_to_edit) {
                        Ok(1) => return Ok(()),
                        Err(e) => e,
                        _ => NoteError::UnexpectedResultError(
                            "Note editing operation should only result in 1 note changed"
                                .to_string(),
                        ),
                    };
                }
            }
        }
    }

    Ok(())
}
