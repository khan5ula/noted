use ansi_term::Colour::Blue;
use clap::{Parser, Subcommand};

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

    /// View newest note(s)
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

    /// View oldest note(s)
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
            long_help = "Delete note(s) by providing a note ID. You can also use the beginning of an ID, eg. f1a8. The command removes all notes with a matching start of an ID."
        )]
        id: String,
    },

    /// Search for a note
    #[command(alias = "s")]
    Search {
        #[arg(help = "Search term to find notes")]
        term: String,
    },
}
