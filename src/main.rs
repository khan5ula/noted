use clap::Parser;
use noted::note;
use noted::note::NoteError;
use noted::SortOrder;

mod args;
mod db;
mod helpers;

use args::*;
use db::*;

fn main() -> Result<(), NoteError> {
    let conn = init_db()?;
    let args = NotedArgs::parse();
    handle_args(conn, args)
}
